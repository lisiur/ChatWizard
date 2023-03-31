# AskAI

AskAI is an OpenAI chat client desktop app (Windows, MacOS, Linux)

## Screenshots

![](./assets/live.gif)
![](./assets/chat.jpeg)
![](./assets/chat-config.jpeg)
![](./assets/prompt.jpeg)
![](./assets/prompt-market.jpeg)
![](./assets/prompt-market2.jpeg)
![](./assets/settings.jpeg)

## Features

- support config chat params
    - model
    - max backtrack
    - temperature
    - max tokens
    - presence penalty
    - frequency penalty
- support prompt
- support prompt market
- support multiple language
    - English
    - Chinese
- support proxy
- support theme
- support forward openai api

## Installation

Download [latest release](https://github.com/lisiur/askai/releases)

## Q&A
1.  MacOS users may encounter this problem: `"askai.app" is damaged and can't be opened. You should move it to the Trash.`

    ```shell
    xattr -cr /Applications/askai.app
    ```

## Dev

1. generate icons

    ```bash
    cd gui && cargo tauri icon icons/app-icon.png
    ```
