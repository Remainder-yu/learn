// use std::num::ParseIntError;
// fn add_two(n_str: &str) -> Result <i32 , ParseIntError> {
//     n_str.parse::<i32>().
// }

// fn main(){
//     assert_eq!(add_two("4").unwrap(),6);
//     println!("sucess");
// }

use crossbeam_deque::Worker;
extern crate crossbeam_learn;

fn main() {
    // 通过闭包和函数分别实现自增。
    // 译注：下面这行是使用函数的实现
    fn  function            (i: i32) -> i32 { i + 1 }

    // 闭包是匿名的，这里我们将它们绑定到引用。
    // 类型标注和函数的一样，不过类型标注和使用 `{}` 来围住函数体都是可选的。
    // 这些匿名函数（nameless function）被赋值给合适地命名的变量。
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    // 译注：将闭包绑定到引用的说法可能不准。
    // 据[语言参考](https://doc.rust-lang.org/beta/reference/types.html#closure-types)
    // 闭包表达式产生的类型就是 “闭包类型”，不属于引用类型，而且确实无法对上面两个
    // `closure_xxx` 变量解引用。

    let i = 1;
    // 调用函数和闭包。
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 没有参数的闭包，返回一个 `i32` 类型。
    // 返回类型是自动推导的。
    let one = || 1;
    println!("closure returning one: {}", one());

    let w = Worker::new_lifo();

    assert!(w.is_empty());
    w.push(1);
    assert!(!w.is_empty());
    let _y = crossbeam_learn::add(2,3);
    crossbeam_learn::test_crossbeam_main();
    crossbeam_learn::test_channel_main();
}
