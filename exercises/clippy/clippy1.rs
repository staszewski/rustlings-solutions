// clippy1.rs
// The Clippy tool is a collection of lints to analyze your code
// so you can catch common mistakes and improve your Rust code.
//
// Execute `rustlings hint clippy1` for hints :)

fn main() {
    let error_margin = std::f64::EPSILON;
    let x = 1.2331f64;
    let y = 1.2332f64;
    if (y - x).abs() > error_margin {
        println!("Success!");
    }
}
