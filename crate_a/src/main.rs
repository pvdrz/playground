fn main() {
    let point = crate_a::Point {
        x: 0,
        y: 0,
        #[cfg(feature = "feat")]
        data: crate_b::random_data(),
    };

    println!("{point:?}");
}
