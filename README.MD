# cp16-rs

[CP16](https://qikan.cqvip.com/Qikan/Article/Detail?id=7105247366) implementation in Rust.

CP-16 本质上就是 16 个正弦波叠加，用于直接在频谱图上显示汉字。

目前本程序通过生成 16 个 PCM 正弦波进行叠加，初步验证了该方法的可行性。接下来需要利用[凤凰点阵体](https://timothyqiu.itch.io/vonwaon-bitmap)生成该类编码的音频信号，同时添加初始生成频率，生成间隔等功能。

顺便，用这个程序学下 rust 的迭代器。