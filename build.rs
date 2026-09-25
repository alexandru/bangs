// Embeds the git commit SHA into the binary, the way the old Gradle
// `generateGitCommitSha` task did: the first 7 characters of
// `git rev-parse HEAD`.

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    let commit_sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|sha| sha.trim().chars().take(7).collect::<String>())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo:rustc-env=BANGS_BUILD_GIT_COMMIT_SHA={commit_sha}");
}
