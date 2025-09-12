use hashmap::Map;

fn main() {
    let mut m: Map<String, i32> = Map::new();

    m.insert("one".to_string(), 1);
    m.insert("two".to_string(), 2);
    m.insert("three".to_string(), 3);

    println!("{m:#?}");

    m.remove("two");
    println!("\nAfter remove\n{m:#?}");
}
