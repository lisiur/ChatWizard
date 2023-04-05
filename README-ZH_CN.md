<p align="center">
  <img width="200" src="./assets/logo.png" alt="ChatWizard">
  <h1 align="center">ChatWizard</h1>
  <p align="center">OpenAI 聊天桌面客户端 (Windows, MacOS, Linux)</p>
</p>

<div align=center>
  <div align=center>
  </div>
  <div>
    <img src="https://img.shields.io/github/package-json/v/lisiur/ChatWizard" />
    <img src="https://visitor-badge.glitch.me/badge?page_id=lisiur.ChatWizard" />
    <img src="https://img.shields.io/github/downloads/lisiur/ChatWizard/total" />
  </div>
  <div>
    <a href="https://github.com/Synaptrix/ChatGPT-Desktop/releases/latest">
      <img alt="macOS" src="https://img.shields.io/badge/-macOS-black?logo=apple&logoColor=white" />
    </a>
    <a href="https://github.com/Synaptrix/ChatGPT-Desktop/releases/latest">
      <img alt="Windows" src="https://img.shields.io/badge/-Windows-blue?logo=windows&logoColor=white" />
    </a>
    <a href="https://github.com/Synaptrix/ChatGPT-Desktop/releases/latest">
      <img alt="Linux" src="https://img.shields.io/badge/-Linux-yellow?logo=linux&logoColor=white" />
    </a>
  </div>
</div>

## 声明

目前该项目正在积极开发中，且只在MacOS M1上进行了测试。如果您遇到任何问题，请不要犹豫立即提交issue，我将尽力解决。此外，欢迎并感激各位以PR的形式作出贡献。

## 截图

<details>
<summary>查看</summary>
<img src="./assets/chat.jpeg" />
<img src="./assets/chat-menus.jpeg" />
<img src="./assets/chat-config.jpeg" />
<img src="./assets/prompt.jpeg" />
<img src="./assets/prompt-menus.jpeg" />
<img src="./assets/prompt-market.jpeg" />
<img src="./assets/prompt-market-menu.jpeg" />
<img src="./assets/setting.jpeg" />
<img src="./assets/light-theme.jpeg" />
</details>


## 特性

- 支持流式回复
- 支持聊天参数配置
- 支持聊天记录的懒加载
- 支持本地提示词
- 支持提示词市场（提示词市场源为 github 的地址，国内用户可能需要设置代理）
- 支持多种语言
- 支持代理
- 支持主题
- 支持转发 openai 的 api

## 安装

- **Mac**

    - [Intel](https://github.com/lisiur/ChatWizard/releases/download/v0.0.42/ChatWizard_0.0.42_x64.dmg)
    - [Apple Silicon](https://github.com/lisiur/ChatWizard/releases/download/v0.0.42/ChatWizard_0.0.42_aarch64.dmg)

    > MacOS 可能会遇到这个问题: `"ChatWizard.app" 已损坏，无法打开。 您应该将它移到废纸篓。`
    > 
    > 打开终端输入：
    > 
    > ```shell
    > xattr -cr /Applications/ChatWizard.app
    > ```

- **Windows**: 

    - [msi](https://github.com/lisiur/ChatWizard/releases/download/v0.0.42/ChatWizard_0.0.42_x64_en-US.msi)

- **Linux**
    - [deb](https://github.com/lisiur/ChatWizard/releases/download/v0.0.42/chat-wizard_0.0.42_amd64.deb)
    - [AppImage](https://github.com/lisiur/ChatWizard/releases/download/v0.0.42/chat-wizard_0.0.42_amd64.AppImage)

## 开发

- 启动

    ```bash
    # root
    pnpm install
    pnpm run install
    pnpm run dev
    ```

- 打包

    ```bash
    pnpm run build
    ```