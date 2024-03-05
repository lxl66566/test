# wordinfo

[English](../README.md) | 简体中文

wordinfo 是一个命令行工具，用于从网络上获取单词的详细信息，包括读音，释义，例句等。

它是 [bakadict](https://github.com/flaribbit/bakadict) 的补充：bakadict 从本地词典中查找单词，而 wordinfo 从网络中获取单词信息；bakadict 仅支持日语，而 wordinfo 支持多种语言。（当前仅支持英语、日语，但可扩展至其他语言。）

wordinfo 允许您编写自己的 [CSS Selector](https://developer.mozilla.org/zh-CN/docs/Web/CSS/CSS_selectors) 以适应不同语言、不同网站的单词查询。

## 安装

前往 [Releases](https://github.com/lxl66566/wordinfo/releases) 下载对应平台的可执行文件，或使用 [bpm](https://github.com/lxl66566/bpm) 安装。

## 使用

```sh
Usage: wordinfo.exe [OPTIONS] <WORDS>...
       wordinfo.exe <COMMAND>

Commands:
  config  （暂未实现）
  export  导出配置文件
  help    打印帮助信息

Arguments:
  <WORDS>...  需要查询的单词

Options:
  -e, --english              英语（默认）
  -j, --japanese             日语
  -s, --selector <SELECTOR>  使用哪个 CSS Selector 来获取单词信息
  -n, --no-url               不输出 URL
  -h, --help                 打印帮助信息
  -V, --version              打印版本信息

Examples:
wordinfo apple              # 查询 apple，= `wordinfo -e apple`
wordinfo -j すき            # 查询 すき，= `wordinfo -j suki`
wordinfo -s oxf apple       # 指定使用 oxford selector 查询 apple
```

wordinfo 支持多种语言，默认使用英语。查询日语时，输入罗马音能够进行假名转换。

`wordinfo -s afs...`（随便输点什么）可以查询当前可用 CSS Selector。指定 CSS Selector 时可以简写，例如 `wordinfo -s oxf ...` 等价于 `wordinfo -s oxford ...`。

## 配置

执行 `wordinfo export` 导出默认配置文件到 `~/.config/wordinfo.toml`（Windows: `%userprofile%/.config/wordinfo.toml`），编辑配置项即可。

目前可调的项目有：

```
default_language:   默认语言
color:              每部分输出的默认颜色
en:                 英语 CSS Selectors
jp:                 日语 CSS Selectors
```

如果要调整默认使用的 CSS Selector，只需将其放到最前即可。（每三个连续的 `[[en]]`, `[en.selector]`, `[en.delimiter]` 属于一个 CSS Selector 整体，需要一起移动）
