use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    println!("{}", if a * b % 2 == 0 { "Even" } else { "Odd" });
}
