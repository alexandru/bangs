use std::process::Command;

fn main() {
    // Get the git commit SHA
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute 'git rev-parse HEAD'. Make sure git is installed and this is a git repository.");

    let git_hash = String::from_utf8(output.stdout)
        .expect("Git command returned invalid UTF-8")
        .trim()
        .chars()
        .take(7)
        .collect::<String>();

    println!("cargo:rustc-env=GIT_COMMIT_SHA={}", git_hash);
}
