# 单元测试

直接在源码文件内编写测试函数，并以 `#[test]` 标记。

```rust
// add.rs

fn add(x: i32, y: i32) -> i32 {
    x + y
}


#[test]
fn add_test() {
    assert_eq!(add(1, 2), 3);                    // 用断言判断测试结果。
}

#[test]
fn result_test() -> Result<(), &'static str> {   // 以 Result 返回测试结果。
    if 2 + 2 == 4 { return Err("abc"); }
    Ok(())
}
```

&nbsp;

确保该模块直接或间接导入根模块。

* `go test -- --ignored`: 测试被忽略的函数。
* `go test <name>`: 指定名称测试。(函数部分名，或 `my::add`这样的模块名)

```rust
$> cargo test
   Compiling ddd v0.1.0 (/root/rs/ddd)
    Finished test [unoptimized + debuginfo] target(s) in 6.04s
     Running /root/rs/ddd/target/debug/deps/ddd-af3dc66fefca6c35

running 2 tests
test add_test ... ok
test result_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

&nbsp;

## 组织

或者组织一下，将该源码文件内所有测试函数放入专门模块。

> 子模块可访问父模块私有成员。

&nbsp;

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(1, 2), 3);
    }
}
```

```rust
$> cargo test
   Compiling ddd v0.1.0 (/root/rs/ddd)
    Finished test [unoptimized + debuginfo] target(s) in 5.89s
     Running /root/rs/ddd/target/debug/deps/ddd-af3dc66fefca6c35

running 1 test
test tests::add_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

&nbsp;

## 注释

除测试函数外，还有一些专用注释。

```rust
#[test]
#[should_panic]  // #[should_panic(expected = "abc")]
fn abc_test() {
    abc();
}

#[test]
#[ignore]        // cargo test -- --ignored
fn ign_test() {
}
```

&nbsp;

## 集成测试

继承测试(integration test)是外部测试，只能针对公开成员。

* 确保测试目标是library。（可以与`main.rs`共存）
* 确保要测试模块已导入 `lib.rs`。
* 创建与 src 同级目录tests。
* 创建测试文件，如 `my_add_test.rs`。

每个测试文件都被当作独立 crate 编译。

```rust
$ tree
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── lib.rs
│   ├── main.rs
│   ├── my
│   │   └── add.rs
│   └── my.rs
└── tests
    └── my_add_test.rs
```

```rust
// src/lib.rs

pub mod my;
```

```rust
// tests/my_add_test.rs

use demo::my::add::*;

#[test]
fn add_test() {
    assert_eq!(add(1, 2), 3);
}
```

&nbsp;

[Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html), [Testing](https://doc.rust-lang.org/1.7.0/book/testing.html)


# 基准测试

官方基准测试库还是实验状态，使用前要先安装。

代码和单元测试类似。

```rust
// lib.rs

#![feature(test)]      // 启用!!!
extern crate test;

mod add;
```

```rust
// add.rs

fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn add_test() {
        assert_eq!(add(1, 2), 3);
    }

    #[bench]
    fn add_bench(b: &mut Bencher) {
        b.iter(|| add(1, 2));
    }

}
```

&nbsp;

命令行添加`+nightly`，否则会出错。

```rust
$ cargo +nightly bench --lib

running 2 tests
test add::tests::add_test ... ignored
test add::tests::add_bench ... bench:           0 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 1 ignored; ... finished in 3.49s


$ cargo +nightly test --lib

running 2 tests
test add::tests::add_bench ... ok
test add::tests::add_test ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; ... finished in 0.00s
```

&nbsp;

## 集成测试

使用 `src` 同级的 benches 目录，每个文件都独立编译。

```rust
$ tree
.
|-- Cargo.lock
|-- Cargo.toml
|-- benches
|   `-- add_test.rs
`-- src
    |-- add.rs
    |-- lib.rs
    `-- main.rs
```

```rust
// src/lib.rs

pub mod add;
```

```rust
// src/add.rs

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

```rust
// benchs/add_test.rs

#![feature(test)]
extern crate test;

use eee::add::*;
use test::Bencher;

#[bench]
fn add_bench(b: &mut Bencher) {
    b.iter(|| add(1, 2));
}
```

```rust

$ cargo +nightly bench
   Compiling eee v0.1.0 (/root/rs/eee)
    Finished bench [optimized] target(s) in 9.43s
     Running unittests (target/release/deps/eee-3e6381fe1f10ca0a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/release/deps/eee-f2a605acbd6dad91)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests (target/release/deps/add_test-97a3bfeb168b555b)

running 1 test
test add_bench ... bench:           1 ns/iter (+/- 0)

test result: ok. 0 passed; 0 failed; 0 ignored; 1 measured; 0 filtered out; finished in 0.75s
```

&nbsp;

[Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html), [Benchmark Tests](https://doc.rust-lang.org/1.7.0/book/benchmark-tests.html)
