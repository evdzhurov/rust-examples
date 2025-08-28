fn decode_float(f: f32) {
    println!("decoding {}", f);

    // Sign
    let sign = f.to_bits() >> 31;
    println!("sign bit: {}", sign);

    // Exponent
    let bias = 127;
    let exp_bits = (f.to_bits() >> 23) & 0xff;
    println!("exp: {}", (exp_bits as i32) - bias);

    // Mantissa
    // - if exponent bits are 0 -> mantissa represents subnormal numbers
    // - if exponent bits are all 1 -> decimal number is infinity, negative
    // infinity or NAN.
    // - represented by bits [0..23), where each bit has weight 2^-i
    // - 24th bit is implicit -> mantissa = 2^-0 = 1.0

    // |s|exp(8)  |mantissa(23)           |
    // |0|00000000|00000000000000000000000|
    //          1. ^                     ^
    //  (implicit) 2^-1                  2^-23

    let mut mantissa = 1.0;
    for i in 0..23 {
        let bit_at_pos_i = f.to_bits() & (1 << i);
        if bit_at_pos_i != 0 {
            let weight = 2_f32.powf(i as f32 - 23.0);
            mantissa += weight;
        }
    }

    println!("mantissa: {}", mantissa);

    println!();
}

fn main() {
    decode_float(-1.2345);
    decode_float(42.42);
    decode_float(1.0 / 256.0);
}
