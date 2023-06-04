// extern crate remainder::*;

struct Point{
    x: i32,
    y: i32,
    z: i32,
}

// 数据可以多次不可变借用，但是在不可变借用的同时
// 原始数据不能使用可变借用
// 同一时间内，只能由一次可变借用
fn main() {
    println!{"hello world"};
    
    let mut point = Point{x:0, y:0, z:0};
    let borrowed_point = &point;
    let another_borrow = &point;

    println!("point has coordinates: ({},{},{})",
                borrowed_point.x, point.y, another_borrow.z);
    
    // let mutable_borrow = &mut point;
    println!("{},{},{}",borrowed_point.x,another_borrow.y, point.z);

    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 6;
    mutable_borrow.z = 7;

    println!("point has coordinates: ({}, {}, {})",
                mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    let new_borrowed_point = &point;
    println!("ponit has coordinates: ({}, {}, {})",
                new_borrowed_point.x,new_borrowed_point.y,new_borrowed_point.z);

}