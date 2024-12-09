#![feature(let_chains)]

use arboard::Clipboard;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use serde::{Deserialize, Serialize};
use std::{env, fmt::Display, io::Read, process::Command};
#[derive(Debug, Serialize, Deserialize)]
struct Msg {
    r#type: String,
    scope: Option<String>,
    subject: String,
    body: Option<String>,
}

impl Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.r#type)?;
        if let Some(scope) = &self.scope {
            if !scope.is_empty() {
                write!(f, "({scope})")?;
            }
        }
        writeln!(f, ": {}", self.subject)?;
        if let Some(body) = &self.body {
            writeln!(f, "\n{body}")?;
        }
        Ok(())
    }
}

const PROMPT: &str = include_str!("../prompt/diff.md");

#[tokio::main]
async fn main() {
    if let Err(e) = diff().await {
        eprintln!("{}", e);
    };
}

async fn diff() -> Result<(), Box<dyn std::error::Error>> {
    let api_base =
        env::var("GMSMG_API_BASE").unwrap_or_else(|_| "https://gmsmg.orzoz.com/v1".to_string());
    let api_model = env::var("GMSMG_API_MODEL").unwrap_or_else(|_| "o1-preview".to_string());
    let api_key = env::var("GMSMG_API_KEY").unwrap_or_else(|_| "".to_string());
    let prompt_file = env::var("GMSMG_PROMPT_FILE").unwrap_or_else(|_| "".to_string());
    let prompt = if !prompt_file.is_empty() {
        let mut file = std::fs::File::open(prompt_file)?;
        let mut diff = String::new();
        file.read_to_string(&mut diff)?;
        diff
    } else {
        PROMPT.to_string()
    };

    // first check if there are any cached changes in the working tree
    let output = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .expect("please install git");
    if !output.status.success() {
        return Err("git diff --cached failed".into());
    }
    let mut diff = String::from_utf8_lossy(&output.stdout).to_string();
    if diff.is_empty() {
        // if there are no cached changes, check for unstaged changes
        let output = Command::new("git")
            .arg("diff")
            .output()
            .expect("please install git");
        if !output.status.success() {
            return Err("git diff failed".into());
        }
        diff = String::from_utf8_lossy(&output.stdout).to_string();
        if diff.is_empty() {
            return Err("no changes to commit".into());
        }
    };

    let mut config = OpenAIConfig::default().with_api_base(api_base);
    if !api_key.is_empty() {
        config = config.with_api_key(api_key);
    }
    let client = async_openai::Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(1000u32)
        .model(api_model)
        .temperature(0.6)
        .top_p(1.0)
        .frequency_penalty(0.6)
        .presence_penalty(1.0)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(diff)
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

    let content = response.choices[0]
        .message
        .content
        .as_ref()
        .ok_or("empty content")?;

    let msg: Msg = serde_json::from_str(content)?;
    println!("{}", msg);
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(msg.subject)?;
    Ok(())
}
