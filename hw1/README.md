# Myfind 

## 如何在本地构建使用

```
$ git clone https://github.com/Moratoryvan/Rust-hw.git
$ cd Rust-hw/hw1
$ cargo build 
```

可执行文件 `myfind` 随后会出现在 `target/debug` 下。


## 用法

```
./myfind -v -path [path] -name [expression]
```

## 功能

* 支持多路径查找，多表达式查找查找的结果是各个路径上对于每一个表达式查询结果的交集，
* 搜索结果命令行色彩输出
* 支持输出所有遍历到的文件

## 使用方式

终端命令: `<myfind 的路径> [-v|--verbose] [-path] <目标目录> [-name] <表达式>`

* `[-v|--verbose]`(optional)
  *  输出所有遍历到的文，必须在开头。
* `[-path]`
  * 后面跟目标目录的路径，必须用在 `-name` 之前。`-path` 之后，`-name` 之前的所有字符串都将被视为目标目录。
* `<目标目录>`
  * 程序会以这个目录为根目录开始查找 
* `[-name]`
  * 后面跟表达式，必须用在 `-path` 之后。 `-name`之后的字符串都将被视为表达式。
* `<表达式>`
  * 程序会以这个表达式为关键字进行查找。

## 文件路径

```
hw1
├─ src
│  ├─ find.rs             // 实现查找
│  ├─ intersec.rs         // 实现 Vector 的交运算
│  └─ main.rs             // 执行具体的 myfind 模式
├─ Cargo.lock
├─ Cargo.toml
└─ README.md
```

