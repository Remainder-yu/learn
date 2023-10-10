# 组织结构示意图

```bash

      +===========+
      | workspace |  工程管理（多项目）
      +===========+
            |
            |      +=========+
            +----- | package |  项目管理（依赖、构建）
            |      +=========+
            |           |
            +-- ..      |       +=======+
            |           +------ | crate |  项目（库，源文件组织）
            |           |       +=======+
                        |           |
                        +-- ..      |      +========+
                        |           +----- | module |  模块（代码组织）
                        |           |      +========+
                                    |
                                    +-- ..
                                    |

```

# 工作空间

包除自己的 Cargo.toml 外, 还可通过工作空间(workspace)共享设置。

```bash
; 换成你需要的名字。
$ mykdir workspace
$ cd workspace

; 创建多个包。
$ cargo new mylib --lib
$ cargo new my

; 创建工作空间配置，添加包成员。
$ cat > Cargo.toml << end
[workspace]
members = ["mylib", "my"]
end

; 编译所有包。
$ cargo b --all
   Compiling mylib v0.1.0
   Compiling my v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 7.44s
```

```toml
# workspace/Cargo.toml

[workspace]
members = ["mylib", "my"]
```

```bash
$ tree
.
├── Cargo.toml
├── my
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── mylib
    ├── Cargo.toml
    └── src
        └── lib.rs
```

&nbsp;

用代码测试一下。

```rust
// mylib/src/lib.rs

pub fn hello() {
    println!("hello, world!");
}
```

```rust
// my/src/main.rs

use mylib;

fn main() {
    mylib::hello();
}
```

&nbsp;

在my配置里添加依赖。

```toml
# my/Cargo.toml

[dependencies]
mylib = { path = "../mylib" }
```

&nbsp;

编译, 运行!

* 相关命令直接在工作空间目录执行。
* 生成的文件在工作空间target目录。

```bash
$ cargo clean

$ cargo b --all
   Compiling mylib v0.1.0
   Compiling my v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 7.22s

$ cargo r
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/my`
hello, world!
```
# 包

**箱子**(crate)对应可执行或库项目；**包**(package)则管理一到多个箱子。

**规则:**

* 最少有一个箱子。
* 最多只能有一个library箱子。
* 可以有任意个binary箱子。

```bash
$> cargo new my --lib
     Created library `my` package

$> tree my
my
|-- Cargo.toml
`-- src
    `-- lib.rs

1 directory, 2 files
```

&nbsp;

> 可选择 `--bin`、`--lib`模版，或在已有目录执行 `init` 命令。
>
> 某些名字(如`test`)有特定意义，不能作为包名。

```toml
[package]
name = "demo"
version = "0.1.0"
authors = []
edition = "2018"

[dependencies]
byteorder = "0.4.0"
num = "0.1.27"

[profile.release]
panic = "abort"

```

[The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)

&nbsp;

## 编译配置

通过配置指定编译参数，分别为:

* `profile.dev`: `cargo run, build`
* `profile.release`: `cargo build --release`
* `profile.test`: `cargo test`
* `profile.bench`: `cargo bench`

[Cargo Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

&nbsp;

## 依赖管理

如果引入第三方包，需要`dependencies`添加依赖设置。

&nbsp;

> 相关工具会自动下载并缓存到`~/.cargo/registry`目录下。
>
> 首次构建时，`Cargo.lock`记录依赖信息。至于在修改依赖版本，或运行`cargo update`时才更新。

无需再做额外声明，直接以use语句引入成员。

```toml
# Cargo.toml

[dependencies]
futures = "0.3"
```

```rust
// main.rs

use futures::executor::block_on;

async fn hello() {
    println!("hello, world");
}

fn main() {
    block_on(hello());
}
```

&nbsp;

## 版本兼容性规则

* `0.0`: 不与任何版本兼容。
* `0.x`: 与`0.x`兼容。(0.61 -> 0.63)
* `1.0`: 主版本号保持兼容。(2.01 -> 2.99, not 3.0)

```toml
image = "=0.10.0"
image = ">=1.0.5"
image = ">1.0.5 <1.1.9"
image = "<=2.7.10"
```

&nbsp;

## 自定义下载路径

非cragtes.io包，可手工指定路径。

```toml
image = { git = "https://github.com/Piston/image.git", branch = "master" }
iamge = { git = "https://github.com/Piston/image.git", rev = "528f19c" }
iamge = { path = "./vendor/image" }
```
# 箱

**箱子**(crate)是一个编译单元，分为可执行(binary) 和 库(library)两类。相比这个有些标新立异的名字，我们更习惯称之为 **项目** 或 **库**。

* 以根文件 `main.rs` 或 `lib.rs` 为起点。
* 同时有上述两个根文件,则代表两个共享源文件的箱子。
* 其余可执行根文件，放在 `src/bin` 目录下。

示例:

```bash

$ tree my

my
|-- Cargo.lock
|-- Cargo.toml
`-- src
    |-- bin
    |   |-- abc.rs
    |   `-- demo.rs
    |-- lib.rs
    `-- main.rs

```

```toml
# Cargo.toml

[package]
name = "my"
version = "0.1.0"
authors = []
edition = "2018"
default-run = "my"

[dependencies]
```

&nbsp;

两个根文件，分别代表 `binary` 和 `library` 箱子。

```rust
// lib.rs

pub fn hello() {
    println!("hello, world!");
}

pub fn test(s: &str) {
    println!("lib: {:?}", s);
}
```

```rust
// main.rs

// 不能用 crate::hello。
// crate 代表 main.rs。

use my::hello;

fn main() {
    hello();
}
```

其它 library箱子。

```rust
// bin/demo.rs

use my::test;

fn main() {
    test("src/bin/demo");
}
```

```rust
// bin/abc.rs

use my::test;

fn main() {
    test("src/bin/abc");
}
```

&nbsp;

编译:

```rust
   Compiling my v0.1.0 (/root/rs/my)
     Running `rustc --crate-name my     src/lib.rs      --crate-type lib
     Running `rustc --crate-name demo   src/bin/demo.rs --crate-type bin
     Running `rustc --crate-name abc    src/bin/abc.rs  --crate-type bin
     Running `rustc --crate-name my     src/main.rs     --crate-type bin

    Finished dev [unoptimized + debuginfo] target(s) in 4.47s
```

&nbsp;

[Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)

# 模块

**模块**(module)是命名空间(namespace)，是函数、类型、常量的容器，用来组织和隔离代码。模块可以是一个目录、一个源文件，或者单个文件内嵌套。

* 根文件模块名为`crate`。
* 其他模块以`crate`为起点，按目录层次构成模块树。

* 子模块，用于分组，控制访问权限。
* 默认私有，添加 `pub` 关键字公开。

* 父模块不能访问子模块私有成员，反之可行。
* 同级模块，不能访问其私有成员。
* 同一模块，成员互相公开。

```rust
mod compiler {
    pub mod checker {
        pub fn check() {
            println!("check!");
        }
    }

    pub mod builder {
        pub fn build() {
            println!("builder!");
        }

        pub fn test() {
            super::checker::check();            // 相对路径: 父级
            self::build();                      // 相对路径: 同级
        }
    }
}

fn main() {
    crate::compiler::builder::test();           // 绝对路径
}
```

## 名字引入

使用 `use` 关键字，将其它模块成员引入当前作用域。

* `use mod::member`: 引入其他模块成员，类似 `from module import member`。
* `use mod::member as newname`: 重命名。
* `use mod::{member1, member2}`: 多个成员。
* `user mod::*`: 全部。

```rust
fn main() {
    use crate::compiler::builder::{build, test};

    build();
    test();
}
```

&nbsp;

组合引入多个成员。

```rust
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io}; // !!!!
```

```rust
use std::io;
use std::io::Write;

use std::io::{self, Write};
```

以 `pub use` 引入的名字，可被外部访问。

```rust
mod test {
    pub use std::mem::size_of_val;
}

fn main() {
    assert_eq!(test::size_of_val(&1), 4);
}
```

&nbsp;

## 模块文件

可将模块拆分到不同文件。每个源码文件构成一个**同名模块**，而子目录名则构成**嵌套关系**。

```bash
$ tree
.
├── compiler
│   ├── builder.rs
│   └── checker.rs
├── compiler.rs
└── main.rs
```

&nbsp;

将compiler模块分离到独立文件内，并创建同名子目录保存其内部子模块。

```rust
// compiler.rs

/*
     mod 相当于 include/import，将子模块包含进来，建立所属关系。
     pub 表示对外公开。
 */

pub mod builder;
pub mod checker;
```

```rust
// compiler/checker.rs

pub fn check() {
    println!("check!");
}
```

```rust
// compiler/builder.rs

pub fn build() {
    println!("builder!");
}
```

&nbsp;

使用时，同样需要引入模块。

```rust
// main.rs

/*
    mod <...>; 不能放在语句块内。
    use <...>; 将名字引入所在作用域。
*/

mod compiler;
use compiler::checker::check;
use compiler::builder::build;

fn main() {
    check();
    build();
}
```

> 2015: `mod lib;` 表示 `./lib.rs` 或 `./lib/mod.rs`，类似 Python `__init__.py`。
>
> 2018： 修改了该方案。

> 标准库在所有项目内默认可用，且自动引入几十个常用类型和特征，以便于使用。

&nbsp;

[2018: Path clarity](https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html)

# 导入

我们可以在模块中以嵌套的方式导入(import)元素，这有助于减少导入操作的资源占用。

```rust
use std::sync::{Mutex, Arc, mpsc::channel};
use std::thread;

fn main() {
    let (tx, rx) = channel();

    let join_handle = thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("Received {}", n);
        }
    });

    for i in 0..10 {
        tx.send(i).unwrap();
    }

    join_handle.join().unwrap();
}
```

&nbsp;

## use 和 extern crate 的区别

### extern crate

`extern crate foo`：引入外部库，要想使其生效，还必须在 `Cargo.toml` 的 `dependecies` 段，加上 `xxx="version num"` 这种依赖说明。引入后，相当于一个符号 `xxx`(`use xxx`)，后面直接可以以这个 `xxx` 为根引用这个crate中的item。

```rust
extern crate xxx;

use xxx::yyy::zzz;
```

从Rust 2018开始，在大多数情况下 `extern crate` 不需要了，直接使用 `use crate` 即可。[use paths](https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html#use-paths)

&nbsp;

### use crate

从Rust 2018后， 基本上不再使用 `extern crate`，而是使用 `use crate` 引入外部包。使用 `use` 前，只需要向 `Cargo.toml` 添加外部依赖项即可。


参考：[the cargo book](https://doc.rust-lang.org/cargo/reference/profiles.html)
