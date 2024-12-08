#![feature(let_chains)]

use arboard::Clipboard;
use serde::{Deserialize, Serialize};
use std::process::Command;
#[derive(Debug, Serialize, Deserialize)]
struct Msg {
    r#type: String,
    scope: Option<String>,
    subject: String,
    body: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // first check if there are any cached changes in the working tree
    let output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .expect("please install git");
    if !output.status.success() {
        eprintln!("error: git diff --cached failed");
        return Ok(());
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
            return Ok(());
        }
        stdout = String::from_utf8_lossy(&output.stdout).to_string();
    };
    let client = reqwest::Client::new();
    let resp: Msg = client
        .post("https://gmsmg.orzoz.com")
        .body(stdout)
        .send()
        .await?
        .json()
        .await?;
    print!("{}", resp.r#type);
    if let Some(scoop) = resp.scope
        && !scoop.is_empty()
    {
        print!("({scoop})");
    }
    println!(":{}", resp.subject);
    if let Some(body) = resp.body {
        println!("\n{body}");
    }
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(resp.subject)?;
    Ok(())
}
