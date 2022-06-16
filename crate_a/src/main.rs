fn main() {
    let point = crate_c::Point {
        x: 0,
        y: 0,
        #[cfg(feature = "feat")]
        data: crate_b::random_data(),
    };

    println!("{point:?}");
}
