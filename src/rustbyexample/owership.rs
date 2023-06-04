
fn destroy_box(c: Box<i32>) {
    println!(" destroy a box that contains {}",c);
}

pub fn test_owership() {
    let x = 5u32;
    let _y = x;
    let a = Box::new(5i32);
    let b = a;
    // println!(" contains a = {}",a);
    destroy_box(b);
}

// 当所有权转移时，数据的可变性可能发生改变
#[allow(dead_code)]
fn test_mutable() {
    let immutable_box = Box::new(5i32);
    println!("immutable_box contain {}",immutable_box);

    let mut mutable_box  = immutable_box;
    *mutable_box = 4;
    println!("mutable now value {}", mutable_box);
}

#[allow(dead_code)]
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("destroy box that contains {}",boxed_i32);
}

#[allow(dead_code)]
fn borrow_i32(borrowed_i32: &i32) {
    println!("this int is: {}",borrowed_i32);
}

#[allow(dead_code)]
fn test_borrow() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    {
        let ref_to_i32: &i32 = &boxed_i32;
        // eat_box_i32(boxed_i32);
        borrow_i32(ref_to_i32);

    }
    eat_box_i32(boxed_i32);
}




#[cfg(test)]
mod tests {
    //use super::destroy_box;
    use super::*;

    #[test]
    fn test_enqueue() {
        test_owership();
        test_mutable();
        test_borrow();
    }
}