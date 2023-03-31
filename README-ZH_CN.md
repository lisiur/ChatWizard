<p align="center">
  <img width="200" src="./assets/logo.png" alt="wizard">
  <h1 align="center">wizard</h1>
  <p align="center">OpenAI 聊天桌面客户端 (Windows, MacOS, Linux)</p>
</p>

[![English badge](https://img.shields.io/badge/%E8%8B%B1%E6%96%87-English-blue)](./README.md)
![visitor](https://visitor-badge.glitch.me/badge?page_id=lisiur.wizard)
[![downloads](https://img.shields.io/github/downloads/lisiur/wizard/total.svg?style=flat-square)](https://github.com/lisiur/wizard/releases)

## 声明

目前该项目正在积极开发中，且只在MacOS M1上进行了测试。如果您遇到任何问题，请不要犹豫立即提交issue，我将尽力解决。此外，欢迎并感激各位以PR的形式作出贡献。

## 截图

![](./assets/live.gif)
![](./assets/chat.jpeg)
![](./assets/chat-config.jpeg)
![](./assets/prompt.jpeg)
![](./assets/prompt-market.jpeg)
![](./assets/prompt-market2.jpeg)
![](./assets/settings.jpeg)

## 特性

- 支持聊天参数配置
- 支持本地提示词
- 支持提示词市场
- 支持多种语言
- 支持代理
- 支持主题
- 支持转发聊天接口

## 安装

下载[最新发行版](https://github.com/lisiur/wizard/releases)

## 开发

- 生成图标

    ```bash
    cd gui && cargo tauri icon icons/app-icon.png
    ```
- 启动开发
    ```bash
    pnpm install && cd gui/web && pnpm install
    pnpm dev
    ```

## 常见问题

-  MacOS 可能会遇到这个问题: `"wizard.app" 已损坏，无法打开。 您应该将它移到废纸篓。`

    打开终端输入：

    ```shell
    xattr -cr /Applications/wizard.app
    ```

