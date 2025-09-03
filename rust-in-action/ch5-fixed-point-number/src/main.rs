use ch5_fixed_point_number::Q7;

fn main() {
    let numbers = [
        -2.0, -1.0, -0.75, -0.5, -0.333, 0.0, 0.333, 0.5, 0.75, 1.0, 2.0,
    ];

    for n in numbers {
        let q7n = Q7::from(n);
        println!("{} -> {:?} -> {}", n, q7n, f64::from(q7n));
    }
}
