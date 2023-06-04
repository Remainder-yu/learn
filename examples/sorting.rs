// use std::clone;

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n-1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i,i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

fn main() {
    let mut ve1 = vec![6,5,4,3,2,1];
    let _cloneed = ve1.clone(); //深层次拷贝
    bubble_sort(&mut ve1);
    println!("{:?}",ve1);
    println!("{:?}",_cloneed); 
}