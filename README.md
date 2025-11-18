# dfint-rust-cjk 矮人要塞汉化程序

这是 Viz 基于上游项目 [df-steam-hook-rs](https://github.com/dfint/df-steam-hook-rs) 开发的矮人要塞汉化程序。从 [cf102ca](https://github.com/dfint/df-steam-hook-rs/commit/cf102ca1dfa1a01fbf991f52771927add7f54691) 分叉并重写了几乎所有的模块，仅使用了上游项目的 hook 框架。

本项目仍处于早期开发阶段，当前版本为 **精简版**（在原来**预览版**的基础上做了精简以方便维护），可能会崩溃以及造成存档损坏、丢失！仅供学习交流。

* 本项目为自由软件，源码托管在 [Gitee:vizv/dfint-rust-cjk](https://gitee.com/vizv/dfint-rust-cjk)
* 项目翻译文件托管在 [Gitee:vizv/df-translations](https://gitee.com/vizv/df-translations)

## 兼容版本

仅兼容以下从官方渠道购买的豪华版 (Premium Edition, 或称之为付费版、图形版) 游戏本体：

* Linux/Windows 系统 [**53.XX+** Steam 版](https://store.steampowered.com/app/975370/Dwarf_Fortress/)
* Linux/Windows 系统 [**53.XX+** itch.io 版](https://kitfoxgames.itch.io/dwarf-fortress)
* Linux/Windows 系统 [**53.XX+** 经典版](http://www.bay12games.com/dwarves/)

**注意事项：**

* [经典版](http://www.bay12games.com/dwarves/) (Classic Edition, 或称之为免费版、字符版) 虽然能兼容但不受支持，可能会显示为 itch.io 版
* 任何修改过可执行文件的游戏版本虽然可能兼容但不受支持，由于汉化程序依赖文件中的各种内存地址偏移未来也不会支持
* 冒险模式会有部分汉化，但是不支持，未来可能会尝试 DFHack 插件版的汉化程序来支持
* 本汉化程序不包含游戏本体的拷贝，请自行到 [itch.io](https://kitfoxgames.itch.io/dwarf-fortress) 或 [Steam](https://store.steampowered.com/app/975370/Dwarf_Fortress/) 平台购买

## 使用步骤

1. 备份你的《矮人要塞》存档
2. 确保你的《矮人要塞》处于受支持的 53.XX 以上版本
3. [从这里](https://df.viz.link/)下载最新的汉化程序完整包
4. 解压压缩包内的所有文件和文件夹到游戏根目录
5. 正常启动游戏
6. 游戏中可以使用 Ctrl+F2 热键对汉化进行开关操作

## 已知问题

* 汉化程序会影响游戏性能（降低游戏帧数）
* 仍有少数下层文本穿透到上层或与其他文本重合的情况
* 标签宽度渲染不正确
* 很多文本翻译缺失
* 有些需要居中的翻译未居中对齐
* 对于带有省略内容（以「…」结尾），暂时无法正确匹配和翻译
* 因为默认配置下仍会启用 dfint 旧词典，会出现一些翻译不准确或零星翻译的情况
* 含多义词的组合可能会翻译错误（常见的有 ash logs - 白蜡树原木，可能会被错误的翻译成灰烬原木；pig iron - 生铁，可能会被错误翻译成家猪铁碇）

## 鸣谢

* 上游项目 [df-steam-hook-rs](https://github.com/dfint/df-steam-hook-rs) 提供的 hook 框架
* dfint 旧词典来自 [dfint](https://github.com/dfint/autobuild/blob/main/translation_build/csv/Chinese%20Simplified/dfint_dictionary.csv)
* 新版翻译词典由[矮人要塞中文维基](https://dfzh.huijiwiki.com/)翻译人员提供
* 字体来自 [Noto CJK](https://github.com/notofonts/noto-cjk)
* 逆向脚本来自 [DFHack 的 df_misc 仓库](https://github.com/DFHack/df_misc)
* DFHack 提供的 [dfhooks API chainloader](https://github.com/DFHack/dfhooks)
* 矮人要塞吧 QQ 群、矮人要塞中文维基翻译 QQ 群、DFHack 的 Discord 频道中的各位大佬的帮助
* **尤为感谢B站大佬 [WAN1694](https://space.bilibili.com/32828123/) 对翻译和测试的帮助！目前 50% 以上的翻译由他完成，并参与了几乎所有的测试和在发版前找到了无数 bug。**

## 许可证信息

* 本汉化程序版权归 [shevernitskiy](https://github.com/shevernitskiy) 和 [Viz](https://gitee.com/vizv) 所有，同上游项目一样使用 MIT 自由软件许可证授权，见 [Gitee:vizv/dfint-rust-cjk:LICENSE](https://gitee.com/vizv/dfint-rust-cjk/blob/viz-wip/LICENSE)
* 翻译文件版权归「[矮人要塞中文维基](https://dfzh.huijiwiki.com/)」翻译人员所有，并在 [知识共享署名-非商业性使用 4.0 国际 (CC BY-NC 4.0)](https://creativecommons.org/licenses/by-nc/4.0/deed.zh-hans) 协议下授权，见 [Gitee:vizv/df-translations:LICENSE.md](https://gitee.com/vizv/df-translations/blob/main/LICENSE.md)
* 所用 Noto CJK 字体使用 SIL Open Font License 授权，见 [GitHub:notofonts/noto-cjk:Sans/LICENSE](https://github.com/notofonts/noto-cjk/blob/main/Sans/LICENSE)
* 所使用的开源库见 `Cargo.toml` 和 `Cargo.lock`，其许可证见各自源码仓库中的许可证信息

## 构建

1. 参考 [Rust 程序设计语言 / 入门指南 / 安装](https://rustwiki.org/zh-CN/book/ch01-01-installation.html) 章节完成 rustup 以及 Rust 和 C++ 工具链的安装。
2. 安装 nightly 版本：`rustup install nightly`
3. 构建汉化程序：`cargo build --release`
4. 复制 `target/release` 目录下的 `libdfint_hook.so` 或 `dfint_hook.dll` 到游戏目录，并重命名为 `libdfhooks.so` 或 `dfhooks.dll`
5. 复制 `data` 目录下的 `offsets.txt` 到游戏目录下的 `dfint-data/offsets.txt`
6. 复制 `config.txt.example` 到游戏目录下的 `dfint-data/config.txt`
7. 下载 [矮人要塞中文翻译词典](https://gitee.com/vizv/df-translations) 中的 `translations` 目录到 `dfint-data/translations`
8. 下载 [Noto CJK](https://github.com/notofonts/noto-cjk) 并将 `NotoSansMonoCJKsc-Bold.otf` 字体文件移动到 `dfint-data/fonts/NotoSansMonoCJKsc-Bold.otf`
9. 下载 [dfint 旧词典](https://github.com/dfint/autobuild/blob/main/translation_build/csv/Chinese%20Simplified/dfint_dictionary.csv) 到 `dfint-data/legacy-dictionary.csv`

如果搞不定，那就直接从[这里](https://df.viz.link/)下一个最新版本的压缩包，然后从中解压对应文件
