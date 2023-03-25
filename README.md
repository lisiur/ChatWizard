# AskAI

AskAI is an OpenAI chat client desktop app (Windows, MacOS, Linux)

## Screenshots

![](./assets/chat.png)
![](./assets/chat_setting.png)
![](./assets/prompt.png)
![](./assets/setting.png)

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