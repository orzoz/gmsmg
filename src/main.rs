use std::process::Command;

fn main() {
    // first check if there are any cached changes in the working tree
    let output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .expect("please install git");
    if !output.status.success() {
        eprintln!("error: git diff --cached failed");
        return;
    }
    let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if stdout.is_empty() {
        // if there are no cached changes, check for unstaged changes
        let output = Command::new("git")
            .arg("diff")
            .output()
            .expect("please install git");
        if !output.status.success() {
            eprintln!("error: git diff failed");
            return;
        }
        stdout = String::from_utf8_lossy(&output.stdout).to_string();
    }
    println!("{}", stdout);
}
