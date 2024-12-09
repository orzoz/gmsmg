# Gmsmg
[![Rust](https://img.shields.io/badge/-Rust-orange?logo=rust&style=for-the-badge&logoColor=white)](https://www.rust-lang.org/)[![Cloudflare](https://img.shields.io/badge/-Cloudflare-yellow?style=for-the-badge&color=555555&logo=cloudflare)](https://www.cloudflare.com/)


Summarize changes in the git repository

## Features
- [x] Straight out of the box.
- [x] Autocopy msg
- [x] Configurable

## Usage

![Example](./public/example.png)

msg will be copied to clipboard automatically.

## Configuration

You can set environment variables to configure.

| Variable            | Description  | Default                      |
|---------------------|--------------|------------------------------|
| `GMSMG_API_BASE`    | API base URL | `https://gmsmg.orzoz.com/v1` |
| `GMSMG_API_MODEL`   | API model    | `o1-preview`                 |
| `GMSMG_API_KEY`     | API key      | empty                        |
| `GMSMG_PROMPT_FILE` | Prompt file  | empty                        |

### PS:
1. look up `prompt/diff.md` to write your own prompt.
2. You are free to use the default configuration to use gmsmg api to create other tools.

## Thanks

- [Cloudflare](https://www.cloudflare.com/) for providing the free tier of their DNS service and free workers.