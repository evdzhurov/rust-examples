fn main() {
    // Compile-time check
    if cfg!(target_endian = "little") {
        println!("System is little-endian");
    } else if cfg!(target_endian = "big") {
        println!("System is big-endian");
    } else {
        println!("System has unknown endianness");
    }

    // Runtime check
    fn is_little_endian() -> bool {
        u16::to_ne_bytes(1)[0] == 1
    }

    println!("little endian: {}", is_little_endian());
}
