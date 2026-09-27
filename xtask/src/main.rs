//! Build and packaging tasks. `cargo xtask dist` assembles the deployable
//! static site in dist/: the wasm bundle goes into a directory named after
//! the git commit (cache busting), and the HTML templates get that path
//! substituted for __BANGS_BUILD_TAG__. Everything provisions its own
//! prerequisites.

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde_json::Value;

/// Target the app compiles to.
const WASM_TARGET: &str = "wasm32-unknown-unknown";
/// Binaryen release `wasm-opt` is downloaded from (wasm-pack pins the same).
const BINARYEN_VERSION: &str = "130";
/// Published version of the DWARF-to-source-map converter.
const WASM2MAP_VERSION: &str = "0.1.0";

fn main() -> Result<()> {
    let usage = "usage: cargo xtask <task>\ntasks: dist, build, serve, format, test-wasm, clean";
    let task = env::args().nth(1).unwrap_or_default();
    match task.as_str() {
        "dist" => dist(),
        "build" => build(),
        "serve" => serve(),
        "format" => format(),
        "test-wasm" => test_wasm(),
        "clean" => clean(),
        _ => bail!("{usage}"),
    }
}

/// Assembles the deployable static site in dist/: the wasm bundle with its
/// source map goes into a directory named after the git commit, and the
/// HTML pages reference that directory.
fn dist() -> Result<()> {
    let tag = build_tag();
    build_with_tag(&tag)?;
    let wasm_bindgen = ensure_wasm_bindgen()?;
    let wasm_opt = ensure_wasm_opt()?;
    let wasm2map = ensure_wasm2map()?;
    let bundle = PathBuf::from(format!("dist/bangs-{tag}"));
    let dirs = [
        bundle.as_path(),
        Path::new("dist/search"),
        Path::new("dist/assets"),
    ];
    for dir in dirs {
        fs::create_dir_all(dir)?;
    }
    run(Command::new(&wasm_bindgen)
        .args(["--target", "web", "--no-typescript", "--keep-debug"])
        .arg("--out-dir")
        .arg(&bundle)
        .arg("target/wasm32-unknown-unknown/release/bangs.wasm"))?;
    // cargo-wasm2map writes only into an existing file, and expects its
    // subcommand name as the first argument when invoked directly.
    let pre_map = bundle.join("bangs_bg.pre.map");
    fs::write(&pre_map, "")?;
    run(Command::new(&wasm2map)
        .arg("wasm2map")
        .arg(bundle.join("bangs_bg.wasm"))
        .arg("-m")
        .arg(&pre_map))?;
    fix_source_map(&env::current_dir()?, &pre_map)?;
    // DevTools ignores source maps while the wasm still has DWARF, so the
    // final binary ships without it.
    run(Command::new(&wasm_opt)
        .args(["-Oz", "--enable-bulk-memory"])
        .arg("--input-source-map")
        .arg(&pre_map)
        .arg("--output-source-map")
        .arg(bundle.join("bangs_bg.wasm.map"))
        .args([
            "--output-source-map-url",
            "bangs_bg.wasm.map",
            "--strip-dwarf",
        ])
        .arg(bundle.join("bangs_bg.wasm"))
        .arg("-o")
        .arg(bundle.join("bangs_bg.opt.wasm")))?;
    fs::rename(
        bundle.join("bangs_bg.opt.wasm"),
        bundle.join("bangs_bg.wasm"),
    )?;
    fs::remove_file(&pre_map)?;
    copy("static/main.js", &bundle.join("main.js"))?;
    render("static/index.html", "dist/index.html", &tag)?;
    render("static/search/index.html", "dist/search/index.html", &tag)?;
    copy("static/search.xml", "dist/search.xml")?;
    copy("static/favicon.ico", "dist/favicon.ico")?;
    copy("static/assets/search.svg", "dist/assets/search.svg")?;
    Ok(())
}

/// Compiles the release wasm binary, with the build's git commit SHA
/// embedded as BANGS_BUILD_GIT_COMMIT_SHA.
fn build() -> Result<()> {
    build_with_tag(&build_tag())
}

/// Rebuilds dist/ and serves it at http://localhost:8080/.
fn serve() -> Result<()> {
    dist()?;
    run(Command::new("python3")
        .args(["-m", "http.server", "8080"])
        .arg("--directory")
        .arg("dist"))
}

/// Formats all crates with nightly rustfmt (the comment wrapping options
/// in rustfmt.toml are unstable).
fn format() -> Result<()> {
    ensure_nightly()?;
    // `$CARGO` resolves to the concrete toolchain binary, which rejects the
    // `+nightly` toolchain specifier; go through the rustup proxy instead.
    run(Command::new("rustup").args(["run", "nightly", "cargo", "fmt", "--all"]))
}

/// Runs the test suite on wasm through the wasm-bindgen test harness
/// (see .cargo/config.toml).
fn test_wasm() -> Result<()> {
    ensure_wasm_target()?;
    run(cargo().arg("test").arg("--target").arg(WASM_TARGET))
}

/// Removes build artifacts and dist/.
fn clean() -> Result<()> {
    run(cargo().arg("clean"))?;
    if Path::new("dist").exists() {
        fs::remove_dir_all("dist")?;
    }
    Ok(())
}

fn build_with_tag(tag: &str) -> Result<()> {
    ensure_wasm_target()?;
    run(cargo()
        .arg("build")
        .args(["--release", "--target"])
        .arg(WASM_TARGET)
        .env("BANGS_BUILD_GIT_COMMIT_SHA", tag))
}

/// Installs the wasm32 standard library when rustup manages the toolchain;
/// skipped silently otherwise (the build then fails with rustc's own hint,
/// since targets cannot be added to a rustup-less toolchain).
fn ensure_wasm_target() -> Result<()> {
    let status = Command::new("rustup")
        .args(["target", "add"])
        .arg(WASM_TARGET)
        .status();
    match status {
        Err(_) => Ok(()),
        Ok(status) if status.success() => Ok(()),
        Ok(status) => bail!("`rustup target add {WASM_TARGET}` failed with {status}"),
    }
}

/// Installs the wasm-bindgen CLI at the version Cargo.lock pins for the
/// crate (the CLI and the crate must match exactly).
fn ensure_wasm_bindgen() -> Result<PathBuf> {
    let version = wasm_bindgen_version()?;
    let is_correct = |program: &OsStr| {
        Command::new(program)
            .arg("--version")
            .output()
            .map(|out| String::from_utf8_lossy(&out.stdout).contains(&version))
            .unwrap_or(false)
    };
    if is_correct(OsStr::new("wasm-bindgen")) {
        return Ok(PathBuf::from("wasm-bindgen"));
    }
    let installed = cargo_bin("wasm-bindgen")?;
    if is_correct(installed.as_os_str()) {
        return Ok(installed);
    }
    run(cargo()
        .args(["install", "wasm-bindgen-cli", "--locked", "--force"])
        .arg("--version")
        .arg(&version))?;
    Ok(installed)
}

/// Downloads the official wasm-opt prebuilt once, into the user cache.
fn ensure_wasm_opt() -> Result<PathBuf> {
    let target = binaryen_target(env::consts::OS, env::consts::ARCH)
        .context("no binaryen release for this platform")?;
    let cache = home_dir()?.join(".cache/bangs");
    let wasm_opt = cache.join(format!("binaryen-version_{BINARYEN_VERSION}/bin/wasm-opt"));
    if !wasm_opt.is_file() {
        fs::create_dir_all(&cache)?;
        let url = format!(
            "https://github.com/WebAssembly/binaryen/releases/download/\
             version_{BINARYEN_VERSION}/binaryen-version_{BINARYEN_VERSION}-{target}.tar.gz"
        );
        let tarball = cache.join("binaryen.tar.gz");
        run(Command::new("curl")
            .args(["-fsSL", &url])
            .stdout(fs::File::create(&tarball)?))?;
        run(Command::new("tar")
            .arg("-xzf")
            .arg(&tarball)
            .arg("-C")
            .arg(&cache))?;
        fs::remove_file(&tarball)?;
    }
    Ok(wasm_opt)
}

/// Installs the DWARF-to-source-map converter when missing.
fn ensure_wasm2map() -> Result<PathBuf> {
    let installed = cargo_bin("cargo-wasm2map")?;
    if !installed.is_file() {
        run(cargo()
            .args(["install", "cargo-wasm2map", "--locked"])
            .arg("--version")
            .arg(WASM2MAP_VERSION))?;
    }
    Ok(installed)
}

/// Nightly is only used for rustfmt, so the minimal profile keeps the
/// download small.
fn ensure_nightly() -> Result<()> {
    let listed = Command::new("rustup").args(["toolchain", "list"]).output();
    match listed {
        Err(_) => return Ok(()), // no rustup; `cargo +nightly` fails with its own hint
        Ok(out) => {
            if String::from_utf8_lossy(&out.stdout).contains("nightly") {
                return Ok(());
            }
        }
    }
    run(Command::new("rustup")
        .args(["toolchain", "install", "nightly", "--profile", "minimal"])
        .args(["--component", "rustfmt"]))
}

/// Rewrites the wasm2map source map so it can ship: paths under the
/// repository become relative, and the map embeds the repository's own
/// source contents (DevTools never has to fetch the file). Everything
/// else (standard library, dependencies) gets an empty string instead of
/// the null that wasm-opt's source-map parser rejects.
fn fix_source_map(repo_root: &Path, map_path: &Path) -> Result<()> {
    let raw = fs::read_to_string(map_path)?;
    let mut map: Value = serde_json::from_str(&raw)?;
    let sources: Vec<String> = map
        .get("sources")
        .and_then(Value::as_array)
        .context("source map has no sources")?
        .iter()
        .map(|source| {
            source
                .as_str()
                .context("source map source is not a string")
                .map(str::to_string)
        })
        .collect::<Result<_>>()?;
    let mut labels: Vec<Value> = Vec::with_capacity(sources.len());
    let mut contents: Vec<Value> = Vec::with_capacity(sources.len());
    for source in &sources {
        let path = Path::new(source);
        let (label, embeddable) = match path.strip_prefix(repo_root) {
            Ok(relative) => (relative.to_string_lossy().into_owned(), true),
            Err(_) if path.is_absolute() => (source.clone(), false),
            Err(_) => (source.clone(), true),
        };
        labels.push(Value::String(label));
        let content = if embeddable {
            fs::read_to_string(path).unwrap_or_default()
        } else {
            String::new()
        };
        contents.push(Value::String(content));
    }
    map["sources"] = Value::Array(labels);
    map["sourcesContent"] = Value::Array(contents);
    fs::write(map_path, serde_json::to_string(&map)?)?;
    Ok(())
}

/// First 7 characters of HEAD's SHA; "unknown" outside a git checkout.
fn build_tag() -> String {
    Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|sha| sha.trim().chars().take(7).collect())
        .unwrap_or_else(|| "unknown".to_string())
}

/// The wasm-bindgen CLI version to use, read from Cargo.lock so it cannot
/// drift from the crate's wasm-bindgen dependency.
fn wasm_bindgen_version() -> Result<String> {
    let lock = fs::read_to_string("Cargo.lock")?;
    let lock: toml::Value = toml::from_str(&lock)?;
    let packages = lock
        .get("package")
        .and_then(toml::Value::as_array)
        .context("Cargo.lock has no packages")?;
    for package in packages {
        if package.get("name").and_then(toml::Value::as_str) == Some("wasm-bindgen") {
            let version = package
                .get("version")
                .and_then(toml::Value::as_str)
                .context("wasm-bindgen has no version in Cargo.lock")?;
            return Ok(version.to_string());
        }
    }
    bail!("wasm-bindgen not found in Cargo.lock")
}

/// Asset name of the binaryen release for this platform, if one exists.
fn binaryen_target(os: &str, arch: &str) -> Option<&'static str> {
    match (os, arch) {
        ("linux", "x86_64") => Some("x86_64-linux"),
        ("linux", "aarch64") => Some("aarch64-linux"),
        ("macos", "x86_64") => Some("x86_64-macos"),
        ("macos", "aarch64") => Some("arm64-macos"),
        _ => None,
    }
}

/// Path of `name` in the cargo home bin directory.
fn cargo_bin(name: &str) -> Result<PathBuf> {
    let cargo_home = match env::var_os("CARGO_HOME") {
        Some(home) => PathBuf::from(home),
        None => home_dir()?.join(".cargo"),
    };
    Ok(cargo_home.join("bin").join(name))
}

fn home_dir() -> Result<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .context("no HOME directory")
}

fn cargo() -> Command {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    Command::new(cargo)
}

fn run(command: &mut Command) -> Result<()> {
    let status = command
        .status()
        .with_context(|| format!("running {command:?}"))?;
    if !status.success() {
        bail!("{command:?} failed with {status}");
    }
    Ok(())
}

fn copy(source: &str, destination: impl AsRef<Path>) -> Result<()> {
    let destination = destination.as_ref();
    fs::copy(source, destination)
        .with_context(|| format!("copying {source} to {destination:?}"))?;
    Ok(())
}

/// Copies an HTML template, substituting __BANGS_BUILD_TAG__ with the
/// bundle directory.
fn render(source: &str, destination: &str, tag: &str) -> Result<()> {
    let html = fs::read_to_string(source)?;
    let html = html.replace("__BANGS_BUILD_TAG__", &format!("bangs-{tag}"));
    fs::write(destination, html)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binaryen_target_covers_the_supported_platforms() {
        assert_eq!(binaryen_target("linux", "x86_64"), Some("x86_64-linux"));
        assert_eq!(binaryen_target("linux", "aarch64"), Some("aarch64-linux"));
        assert_eq!(binaryen_target("macos", "x86_64"), Some("x86_64-macos"));
        assert_eq!(binaryen_target("macos", "aarch64"), Some("arm64-macos"));
        assert_eq!(binaryen_target("windows", "x86_64"), None);
    }

    #[test]
    fn fix_source_map_relativizes_and_embeds_only_repo_sources() -> Result<()> {
        let root = env::temp_dir().join(format!("bangs-xtask-test-{}", std::process::id()));
        fs::create_dir_all(root.join("src"))?;
        fs::write(root.join("src/app.rs"), "fn app() {}")?;
        let map_path = root.join("bangs_bg.wasm.map");
        let absolute = root.join("src/app.rs");
        fs::write(
            &map_path,
            format!(
                r#"{{"version":3,"sources":["{}","/nonexistent/outside.rs","library/core/src/str.rs"],"sourcesContent":null,"names":[],"mappings":"AAAA"}}"#,
                absolute.display()
            ),
        )?;
        fix_source_map(&root, &map_path)?;
        let map: Value = serde_json::from_str(&fs::read_to_string(&map_path)?)?;
        assert_eq!(
            map["sources"],
            serde_json::json!([
                "src/app.rs",
                "/nonexistent/outside.rs",
                "library/core/src/str.rs"
            ])
        );
        assert_eq!(
            map["sourcesContent"],
            serde_json::json!(["fn app() {}", "", ""])
        );
        fs::remove_dir_all(&root)?;
        Ok(())
    }
}
