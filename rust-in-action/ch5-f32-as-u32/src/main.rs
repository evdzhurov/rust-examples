fn main() {
    let a: f32 = 42.42;
    println!("f32 value: {}", a);

    // let frankentype: u32 = unsafe { std::mem::transmute(a) };
    let frankentype: u32 = f32::to_bits(a);

    println!("f32 -> u32 value: {}", frankentype);

    // {:032b} formats as binary via the std::fmt::Binary trait
    // with 32 zeroes padded on the left
    println!("u32 bits: {:032b}", frankentype);

    // let b: f32 = unsafe { std::mem::transmute(frankentype) };
    let b: f32 = f32::from_bits(frankentype);
    println!("u32 -> f32: {}", b);

    assert_eq!(a, b);
}
