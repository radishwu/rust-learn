# rust-learn
Rust learning record.

## 常用命令

### Cargo常用命令
```bash
//查看 cargo 版本 
cargo --version 

//创建新项目 
cargo new hello_cargo 

//构建项目 
cargo build 

//构建并执行项目 
cargo run 

//检查代码确保其可以编译 
cargo check 

//构建release版本 
cargo build --release 

//构建所有本地依赖提供的文档，并在浏览器中打开。 
cargo doc --open
```

### Rust常用命令
```bash
//编译.rs文件，输出一个二进制的可执行文件 
rustc main.rs 

//安装最新稳定版本的rust 
rustup
```

## 概念笔记
### 所有权
#### 所有权规则
1. Rust 中的每一个值都有一个**所有者**（owner）。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。
#### 引用的规则
1. 在任意给定时间，**要么** 只能有一个可变引用，**要么** 只能有多个不可变引用。
2. 引用必须总是有效的。