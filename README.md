# 以Zed项目为例学习大型Rust项目的组织与管理

摘要：Zed是一款使用Rust编写由Atom创始人开发的高性能、协作友好的现代开源代码编辑器，Zed项目的组织和管理非常值得学习和研究

说明

* Zed项目代码：https://github.com/zed-industries/zed.git
* 本文项目代码：https://github.com/VinciYan/zed_workspace.git

Zed是一款由Atom创始人开发的高性能、协作友好的现代开源代码编辑器，使用Rust编写，集成AI辅助功能，旨在结合传统编辑器的速度和现代IDE的智能特性

Zed项目的组织和管理非常值得学习和研究。下面我将通过我总结后的得出一个精简版例子来说明Zed项目的结构，my_project对应Zed项目的crates文件夹下的zed文件夹

```plaintext
workspace_root/
├── Cargo.toml
└── crates/
    ├── foo/
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    ├── bar/
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    └── my_project/
        ├── Cargo.toml
        └── src/
            └── main.rs
```

工作空间根目录的Cargo.toml

```toml
[workspace]
members = ["crates/foo", "crates/bar", "crates/my_project"]
default-members = ["crates/my_project"]
resolver = "2"

[workspace.dependencies]
foo = { path = "crates/foo" }
bar = { path = "crates/bar" }
rand = "0.8.5"  # 添加 rand 作为工作空间依赖

[workspace.lints.rust]
unsafe_code = "forbid"
unused_variables = "warn"

[workspace.lints.clippy]
enum_glob_use = "deny"
```

members字段指定了属于该工作空间的所有 crate（包）

* 作用：

  * 定义工作空间结构：告诉Cargo哪些项目是这个工作空间的一部分
  * 允许集中管理：可以在工作空间级别管理依赖和构建设置
  * 启用共享编译：成员之间可以共享编译产物，提高构建效率

* 灵活性：

  * 可以使用相对路径指定成员
  * 支持glob模式，如members = ["crates/*"] 可以包含crates目录下的所有crate

default-members指定了在没有明确指定目标的情况下，默认要构建、测试或运行的工作空间成员（crate）

* 当你在工作空间根目录运行cargo build、cargo test或cargo run等命令时，如果没有指定具体的crate，Cargo会默认对default-members中列出的crate执行操作
* 在配置中，default-members = ["crates/my_project"]意味着默认情况下，只有my_project会被构建或运行
* 设置my_project为默认成员意味着你可以直接在根目录运行cargo run，而不需要指定--package my_project
* 可以指定多个默认成员，例如：default-members = ["crates/my_project", "crates/foo"]

resolver = "2"在工作空间的Cargo.toml文件中是一个重要的配置项，它指定了Cargo使用的依赖解析器版本

* 版本

  * "1": 旧版解析器（默认用于2015和2018版本的Rust）
  * "2": 新版解析器（默认用于2021版本的 Rust）

注意到rand指定项目版本信息，这样的好处包括：

* 在工作空间级别管理依赖版本，确保所有项目使用相同版本的rand
* 简化了各个项目的Cargo.toml文件，使其更加清晰
* 方便统一升级所有项目的依赖版本

需要注意的是，这种方式要求所有使用rand的项目都使用相同的版本。如果某个项目需要使用不同版本的rand，你可以在那个项目的Cargo.toml中明确指定版本，覆盖工作空间中定义的版本

例如，如果bar项目需要使用不同版本的rand：

```toml
[dependencies]
foo.workspace = true # 也可写成旧版本的Cargo中出现的foo = { workspace = true }
rand = "0.7.3"  # 使用特定版本，而不是工作空间版本
```

这种配置方式既灵活又统一，非常适合管理包含多个相关项目的工作空间

lint设置用于定义整个工作空间的代码质量和样式规则。好处：

* 一致性：确保工作空间中所有项目遵循相同的代码规范
* 安全性：通过禁止不安全代码提高项目的安全性
* 代码质量：帮助开发者编写更清晰、更高质量的代码
* 维护性：通过统一的规则，使代码更容易维护和理解

**my_project项目**

Cargo.toml

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "my_project"
path = "src/main.rs"

[dependencies]
foo.workspace = true
bar.workspace = true

[lints]
workspace = true
```

src/main.rs

```rs
use bar::bar_function;
use foo::generate_random_number;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", bar::add_one(num));
    bar_function();
    let random_number = generate_random_number();
    println!("Random number generated in bar: {}", random_number);
}
```

**foo项目**

Cargo.toml

```toml
[package]
name = "foo"
version = "0.1.0"
edition = "2021"

[dependencies]
rand.workspace = true

[lints]
workspace = true

[lib]
path = "src/lib.rs"
doctest = false
```

src/lib.rs

```rs
use rand::Rng;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn foo_function() {
    println!("This is a function from foo");
}

pub fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

**bar项目**

Cargo.toml

```toml
[package]
name = "bar"
version = "0.1.0"
edition = "2021"

[lints]
workspace = true

[lib]
path = "src/lib.rs"

[dependencies]
foo.workspace = true
```

src/lib.rs

```rs
use foo::foo_function;

pub fn bar_function() {
    println!("This is a function from bar library");
    foo_function(); // 调用 foo 中的函数
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

bar项目依赖foo项目，调用foo项目的foo_function函数

执行“Cargo run”，项目运行结果如下

```plaintext
Hello, world! 10 plus one is 11!
This is a function from bar library
This is a function from foo
Random number generated in bar: 13
```

最后，我还给出另一种项目代码组织方式，将my_project项目放到crates文件夹之外的版本，感兴趣的话，详见我的项目仓库

## 参考

1. [Announcing Rust 1.74.0 | Rust Blog (rust-lang.org)](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html)
2. [Workspaces - The Cargo Book (rust-lang.org)](https://doc.rust-lang.org/stable/cargo/reference/workspaces.html#the-lints-table)

‍
