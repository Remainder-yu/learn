use std::collections::HashMap;

#[derive(Debug,Hash,Eq,PartialEq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
     /// Creates a new Viking.
     fn new(name: &str, country: &str) -> Viking {
        Viking { name: name.to_string(), country: country.to_string() }
    }
}

fn main(){
    // Use a HashMap to store the vikings' health points.
    let vikings = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{viking:?} has {health} hp");
    }
    test_main();
    count_letter();
}

fn test_main() {
    let mut play_stats = HashMap::new();
    play_stats.entry("health").or_insert(100);
    println!("{:?}",play_stats.get_key_value("health"));
    println!("{:?}",play_stats.get("health"));

    let mut map = HashMap::new();
    map.insert(1, "a");
    assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    assert_eq!(map.get_key_value(&2), None);

    let mut map1: HashMap<i32, i32> = HashMap::with_capacity(100);
    map1.insert(1, 2);
    println!("map1 = {:?}",map1.get(&1));
}

fn count_letter(){
    let mut letters = HashMap::new();
    for ch in "a short treatise on fun.g".chars() {
        letters.entry(ch).and_modify(|counter|*counter += 1).or_insert(1);
    }
    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
}

