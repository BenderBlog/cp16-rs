# cp16-rs

[CP16](https://qikan.cqvip.com/Qikan/Article/Detail?id=7105247366) implementation in Rust.

CP-16 本质上就是 16 个正弦波叠加，用于直接在频谱图上显示汉字。我觉得这个东西就是某种最简单的正弦波叠加。如果使用复杂模式，比如说更加密集的频率，那就是 CTF Misc 题目，还有 DOOM 2016 中制作人留下的“我爱上帝”彩蛋。

目前本程序通过生成 16 个 PCM 正弦波进行叠加，初步实现了该编码方式，同时添加了初始生成频率，生成间隔等功能。接下来需要处理横向字符生成，并将其写成一个库。在这个库的基础上，编写程序使其可以实时播放或者写入文件。

本程序依赖 GNU Unifont 库，不打算支持外部引入字体。GNU Unifont 库支持 Unicode Basic Mulitlingual Plane，也就是基本上常见的字体都能支持。

顺便，用这个程序学下 rust 的迭代器。