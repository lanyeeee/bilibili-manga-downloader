# 哔哩哔哩漫画下载器

<p align="center">
    <img src="https://github.com/user-attachments/assets/f40919c1-569a-451e-a32f-7b32e8843dc4" width="200" style="align-self: center"/>
</p>



一个用于 哔哩哔哩漫画 B漫 的多线程下载器，带图形界面，支持特典下载，下载速度飞快。图形界面基于[Tauri](https://v2.tauri.app/start/)

在[Release页面](https://github.com/lanyeeee/bilibili-manga-downloader/releases)可以直接下载

**如果本项目对你有帮助，欢迎点个 Star⭐ 支持！你的支持是我持续更新维护的动力🙏**

# 图形界面

![image](https://github.com/user-attachments/assets/09f12266-a3a7-4337-90b9-7be1ae649e88)


# 使用方法

1. 点击`二维码登录`按钮，使用官方APP完成扫码登录(~~手动输入`SESSDATA`也行~~)
2. 使用`漫画搜索`，通过`关键词`搜索漫画，点击漫画的`封面`或`标题`，进入`章节详情`(也可以通过`漫画ID`直达`章节详情`)
3. 在`章节详情`勾选要下载的章节，点击`下载勾选章节`按钮开始下载
4. 下载完成后点击`打开下载目录`按钮查看结果

下面的视频是完整使用流程

https://github.com/user-attachments/assets/dc4d5b63-dba6-4d72-b9d1-c9b0b6f79ef3

# 哔哩哔哩漫画去水印工具

[![Readme Card](https://github-readme-stats.vercel.app/api/pin/?username=lanyeeee&repo=bilibili-manga-watermark-remover)](https://github.com/lanyeeee/bilibili-manga-watermark-remover)   


# 关于被杀毒软件误判为病毒

对于个人开发者来说，这个问题几乎是无解的(~~需要购买数字证书给软件签名，甚至给杀毒软件交保护费~~)  
我能想到的解决办法只有：

1. 根据下面的**如何构建(build)**，自行编译
2. 希望你相信我的承诺，我承诺你在[Release页面](https://github.com/lanyeeee/bilibili-manga-downloader/releases)下载到的所有东西都是安全的

# 关于软件传播

私下传播的软件可能因篡改而携带病毒，为避免用户因使用私下传播的版本而感染病毒，甚至因此来找我麻烦  
我只保证在[Release页面](https://github.com/lanyeeee/bilibili-manga-downloader/releases)下载到的东西是安全的  

若需要私下传播该软件，请务必进行以下操作：
1. 修改软件标识符(`src-tauri/tauri.conf.json`的`identifier`字段)然后重新编译
2. 仅传播经过重新编译的版本，并明确标注这是经过修改的版本

# 如何构建(build)

构建非常简单，一共就3条命令 
~~前提是你已经安装了Rust、Node、pnpm~~

#### 前提

- [Rust](https://www.rust-lang.org/tools/install)
- [Node](https://nodejs.org/en)
- [pnpm](https://pnpm.io/installation)

#### 步骤

#### 1. 克隆本仓库

```
git clone https://github.com/lanyeeee/bilibili-manga-downloader.git
```

#### 2.安装依赖

```
cd bilibili-manga-downloader
pnpm install
```

#### 3.构建(build)

```
pnpm tauri build
```

# 提交PR

**PR请提交至`develop`分支**

**如果想新加一个功能，请先开个`issue`或`discussion`讨论一下，避免无效工作**

其他情况的PR欢迎直接提交，比如：

1. 对原有功能的改进
2. 使用更轻量的库实现原有功能
3. 修订文档
4. 升级、更新依赖的PR也会被接受

# 免责声明

- 本工具仅作学习、研究、交流使用，使用本工具的用户应自行承担风险
- 作者不对使用本工具导致的任何损失、法律纠纷或其他后果负责
- 作者不对用户使用本工具的行为负责，包括但不限于用户违反法律或任何第三方权益的行为

# 其他

任何使用中遇到的问题、任何希望添加的功能，都欢迎提交issue或开discussion交流，我会尽力解决  
