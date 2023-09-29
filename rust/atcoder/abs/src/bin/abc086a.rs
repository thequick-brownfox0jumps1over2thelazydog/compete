use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    }

    let result = if a * b % 2 == 0 { "Even" } else { "Odd" };

    println!("{result}");
}
