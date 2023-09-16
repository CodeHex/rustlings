// async1.rs
// Running a basic async function. Try to get the answer to print out.
// Note that `tokio` with full features have been added to `Cargo.toml` for you already. 
//
// Execute `rustlings hint async1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
     let result = complex_calculation();
     println!("The answer is {result}");
}

// Don't change anything below this line.
async fn complex_calculation() -> i32 {
    2 + 2
}
