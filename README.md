> 注：本人第一次写爬虫，不喜勿喷  

# Bilibili CV Downloader
B站专栏图片批量下载器  

### 下载
Windows: 请到 [Release](https://github.com/jcshan709/bilibili-cv-downloader/releases) 页面下载程序（经过自签名，推荐）  
最新构建/Ubuntu/MacOS: 在 [Actions](https://github.com/jcshan709/bilibili-cv-downloader/actions) 里下载自动构建的最新版（90天保存时间）  
其他情况请自行构建

### 食用方法 
运行之后把**链接**或**CV号**直接粘贴进去，等待完成即可，下载好的图片会存放在工作目录的`cv-images`文件夹下，文件名按顺序命名  

### 构建方法
1. 安装 [Rust](https://www.rust-lang.org/zh-CN/tools/install)
2. 运行 `cargo build --release`
3. 可执行文件位于 `target/release` 目录下
