<div align="center">
  <p style="font-size:40px">rupass</p>
  rupass is a command line and geek style password manager written in Rust.
  <br/>
  <a href="https://crates.io/crates/rupass"><img alt="crates.io" src="https://img.shields.io/badge/crates.io-v0.1.2-brightgreen" /></a>
  <a href="https://blog.rust-lang.org/2022/07/19/Rust-1.62.1.html"><img alt="Rust Version" src="https://img.shields.io/badge/rust-1.62%2B-blue" /></a>  
  <HR align=center width=500 color=“green” SIZE=1>
  </div>

## 🍻 Features

- Built with [Sled](https://github.com/spacejam/sled) 、 [inquire](https://github.com/mikaelmello/inquire) and [clap](https://github.com/clap-rs/clap)
- Offer add、delete、search functions now

## s🧸 Installation

To install rupass, use package manager:

`cargo install rupass`

By default, this command will download the crate and compile the binary file and then copy it to `~/.cargo/bin/`, if you have already set enviroment variables, you can run with `rupass` then.

## 🎡 Usage

Run `rupass --help` to see the details:

![image.png](https://tva1.sinaimg.cn/large/e6c9d24egy1h4nq79eg6yj219k0rqwif.jpg)

### 🌴 Init 
First you need to init your database to store your account info, the default dir is in your home dir: e.g.`~/.rupass`.
```bash
rupass init
```
You need an entry password you must remember, but now, it's no use, maybe in use few days later.
![image.png](https://tva1.sinaimg.cn/large/e6c9d24egy1h4nqagw4fej21hi0c040o.jpg)

### ☘️ Add password
```bash
rupass add
```
You need input info below:
- website/app keywords
- username
- password(default display with `*`, you can show it with `Ctrl+R`)
- notes

![image.png](https://tva1.sinaimg.cn/large/e6c9d24egy1h4nqe5eq7qj21he0f0dij.jpg)

### 🍀 Search password
```bash
rupass search
```
Just input the keyword, there is a suggester for you! You can select which you want.
![image.png](https://tva1.sinaimg.cn/large/e6c9d24egy1h4nqfw4h09j21he0dggnb.jpg)

### 🍁 Delete password
```bash
rupass delete
```
You need input the entire account info, such as `baidu.com<admin>`.
in fact, you don't need to input the entire info, just select from the suggester which you need to delete.
![image.png](https://tva1.sinaimg.cn/large/e6c9d24egy1h4nqhi7po2j21hk0dcac4.jpg)
