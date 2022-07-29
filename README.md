## rupass
This is a command line and geek style password manager written in Rust.

## Usage

### clone the repository and build the project
```bash
# clone
git clone https://github.com/lijing-2008/rupass.git
# build project
cd rupass
cargo build
```
then you can find your own `rupass` in `target/debug/rupass`.
### init your database
First you need to init your database to store your account info, the default dir is in your home dir: e.g.`~/.rupass`.
```bash
target/debug/rupass init
```
You need an entry password you must remember, but now, it's no use, maybe in use few days later.
![](https://github.com/lijing-2008/PicGo/blob/master/init.gif)
### add password
```bash
target/debug/rupass add
```
You need input info below:
- website/app keywords
- username
- password(default display with `*`, you can show it with `Ctrl+R`)
- notes
![](https://github.com/lijing-2008/PicGo/blob/master/add.gif)
### search password
```bash
target/debug/rupass search
```
Just input the keyword, there is a suggester for you! You can select which you want.
![](https://github.com/lijing-2008/PicGo/blob/master/search.gif)
### delete password
```bash
target/debug/rupass delete
```
You need input the entire account info, such as `baidu.com<admin>`.
in fact, you don't need to input the entire info, just select from the suggester which you need to delete.
![](https://github.com/lijing-2008/PicGo/blob/master/delete.gif)

## Custom welcome banner
You can change welcome banner yourself, edit `banner.txt` in root directory.
