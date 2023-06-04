#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut config:Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    // Some(&mut Config { a: "A".to_string(), b: "B".to_string() })
    Some(
        Box::leak(Box::new (
            Config { a: "a".to_string(), b: "b".to_string() }
        ))

    )
}


fn test_main() {
    let static_string = "hello world";
    println!("static_string:{}",static_string);
}

fn main() {
    test_main();
    unsafe {
        config = init();
        println!("{:?}",config);
    }
}