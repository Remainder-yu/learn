
//
fn print_one<'a>(x: &'a i32) {
    println!(" print_one: x is {}",x);
}

fn print_refs<'a, 'b> (x: &'a i32, y: &'b i32 ) {
    println!("x is {} and y is {}",x ,y);
}

// 可变引用同样也拥有生命周期
fn add_one<'a>(x:&'a mut i32){
    *x += 1;
}

// 返回传递进阿里的引用是可行的，但必须返回正确的生命周期
fn pass_x<'a, 'b>(x : &'a i32, _: &'b i32) -> &'a i32 { x }


fn test_print_refs() {
    let(four, nine) = (4, 9);
    print_refs(&four, &nine);
    let mut y = four + nine;
    print_one(&y);
    let mut x = 5;
    add_one(&mut y);
    add_one(&mut x);
    print_one(&x);

    let z = pass_x(&x, &y);
    print_one(&z);
}

// 15.4.3 方法
#[derive(Debug, PartialEq)]
struct Owner(i32);
impl Owner {
    fn add_one<'a>( &'a mut self) {
        self.0 += 1;
    }
    fn print_one<'a> (&'a self) {
        println!("print: {:?}",self.0);
    }
}

fn test_owner() {
    let mut owner_x = Owner(32);
    owner_x.add_one();
    owner_x.print_one();
    assert_eq!(owner_x , Owner(33));
}

//15.4.4 struct 结构体
// 在结构体中标注生命周期也和函数的类似：

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct BorrowedTwo<'a> {
    x: &'a i32,
    y: &'a i32, 
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

// 15.4.5 trait


fn test_struct_lifetime(){
    let x = 19;
    let y = 20;
    let single = Borrowed(&x);
    let double = BorrowedTwo {x: &x, y: &y};
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!(" {:?} , {:?} , {:?} , {:?} ",single,double,reference,number);
}

fn main() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_one(&y);

    test_print_refs();

    test_owner();
    test_struct_lifetime();
}