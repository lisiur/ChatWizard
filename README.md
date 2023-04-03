<p align="center">
  <img width="200" src="./assets/logo.png" alt="ChatWizard">
  <h1 align="center">ChatWizard</h1>
  <p align="center">OpenAI chat client Desktop Application (Windows, MacOS, Linux)</p>
</p>

[![简体中文 badge](https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-Simplified%20Chinese-blue)](./README-ZH_CN.md)
![visitor](https://visitor-badge.glitch.me/badge?page_id=lisiur.ChatWizard)
[![downloads](https://img.shields.io/github/downloads/lisiur/ChatWizard/total.svg?style=flat-square)](https://github.com/lisiur/ChatWizard/releases)

## Declaration

This project is currently undergoing active development and has only been tested on MacOS M1. In the event of any issues, please do not hesitate to submit an issue as they arise, and I will make every effort to address them. Additionally, all contributions in the form of PRs are welcome and greatly appreciated.

## Screenshots

![](./assets/live.gif)
![](./assets/chat.jpeg)
![](./assets/chat-config.jpeg)
![](./assets/prompt.jpeg)
![](./assets/prompt-market.jpeg)
![](./assets/prompt-market2.jpeg)
![](./assets/settings.jpeg)

## Features

- support chat configuration
- support local prompt
- support prompt market
- support multiple language
- support proxy
- support theme
- support forward openai api

## Installation

Download [latest release](https://github.com/lisiur/ChatWizard/releases)

## Development

- generate icons

    ```bash
    cd gui
    cargo tauri icon icons/macos/app-icon.png -o icons/macos
    cargo tauri icon icons/linux/app-icon.png -o icons/linux
    cargo tauri icon icons/windows/app-icon.png -o icons/windows
    ```
- dev
    ```bash
    pnpm install && cd gui/web && pnpm install && cd .. && pnpm dev
    ```

## Q&A

-  MacOS users may encounter this problem: `"ChatWizard.app" is damaged and can't be opened. You should move it to the Trash.`

    open terminal and execute:

    ```shell
    xattr -cr /Applications/ChatWizard.app
    ```

