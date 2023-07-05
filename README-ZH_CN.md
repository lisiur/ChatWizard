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
    <a href="https://github.com/lisiur/ChatWizard/releases/latest">
      <img alt="macOS" src="https://img.shields.io/badge/-macOS-black?logo=apple&logoColor=white" />
    </a>
    <a href="https://github.com/lisiur/ChatWizard/releases/latest">
      <img alt="Windows" src="https://img.shields.io/badge/-Windows-blue?logo=windows&logoColor=white" />
    </a>
    <a href="https://github.com/lisiur/ChatWizard/releases/latest">
      <img alt="Linux" src="https://img.shields.io/badge/-Linux-yellow?logo=linux&logoColor=white" />
    </a>
  </div>
</div>

## 声明

- 所有的数据都只会存储在本地。
- 聊天记录以及 API key 只会用于 OpenAI 的接口，不会发送到任何其他地方。
- 如果担心 API key 泄漏，可以使用应用内的转发地址功能（该功能允许你在自己的服务器上存储 Api key，只需提供一个对 OpenAI 接口的反向代理地址即可）。

## 截图

<img src="./assets/slash-command.png" />

<details>
<summary>查看更多</summary>
<img src="./assets/casual-chat.png" />
<img src="./assets/chat.png" />
<img src="./assets/prompt.png" />
<img src="./assets/prompt-market.png" />
<img src="./assets/setting.png" />
<img src="./assets/tray-window.png" />
</details>


## 特性

- 流式回复
- 停止回复
- 托盘窗口
- 网络代理
- 接口转发
- 聊天记录的懒加载
- 导出图片
- 聊天参数配置
- 提示词市场（提示词市场源为 github 的地址，国内用户可能需要设置代理）
- 本地提示词
- 多种语言
- 亮/暗主题

## 开发中

- [ ] 多用户
- [ ] 共享聊天记录
- [ ] 导出 pdf/图片/markdown

## 安装

> 如果在点击下载链接时出现404错误页面，可能是由于应用程序当前正在打包的缘故。请稍后再试，或者直接从[这里](https://github.com/lisiur/ChatWizard/releases/latest)下载当前最新版本。

- **Mac**

    - [Intel](https://github.com/lisiur/ChatWizard/releases/download/v0.3.0/ChatWizard_0.3.0_x64.dmg)
    - [Apple Silicon](https://github.com/lisiur/ChatWizard/releases/download/v0.3.0/ChatWizard_0.3.0_aarch64.dmg)

    > MacOS 可能会遇到这个问题: `"ChatWizard.app" 已损坏，无法打开。 您应该将它移到废纸篓。`
    > 
    > 打开终端输入：
    > 
    > ```shell
    > xattr -cr /Applications/ChatWizard.app
    > ```

- **Windows**: 

    - [msi](https://github.com/lisiur/ChatWizard/releases/download/v0.3.0/ChatWizard_0.3.0_x64_en-US.msi)

- **Linux**
    - [deb](https://github.com/lisiur/ChatWizard/releases/download/v0.3.0/chat-wizard_0.3.0_amd64.deb)
    - [AppImage](https://github.com/lisiur/ChatWizard/releases/download/v0.3.0/chat-wizard_0.3.0_amd64.AppImage)

    > 如果 linux 版的应用无法启动并不奇怪。这是因为这两个包都是在 ubuntu20 上构建的。或许你可以尝试在自己本地构建。

- **从源码构建**
    1. 首先你需要安装[Tauri 开发环境](https://tauri.app/v1/guides/getting-started/prerequisites)和[pnpm](https://pnpm.io/installation)
    2. 然后执行下面的指令进行构建
        ```bash
        git clone https://github.com/lisiur/ChatWizard.git
        cd ChatWizard
        pnpm install
        pnpm run install
        pnpm run build
        ```
    3. 你的本地构建版本会在 target/release/bundle/<本地系统> 文件夹里

## 升级

ChatWizard 支持内置升级，每当你重新启动应用时都会自动查询是否有新版本发布，如果有新版本会在左下角显示小红点。

升级时会从 GitHub 上下载更新包，对于国内用户来说，可能需要设置代理。但同时由于 Tauri 的限制目前尚不支持在应用中使用应用设置的代理升级。此时你可以尝试下面的方法：

- **Mac**
  
  退出应用并打开终端输入：

  ```
  https_proxy=<你的代理地址> /Applications/ChatWizard.app/Contents/MacOS/ChatWizard
  # 最终的命令像是这样：https_proxy=http://127.0.0.1:7890 /Applications/ChatWizard.app/Contents/MacOS/ChatWizard
  ```
  该命令会重新打开应用，此时点击升级会使用你设置的代理。

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