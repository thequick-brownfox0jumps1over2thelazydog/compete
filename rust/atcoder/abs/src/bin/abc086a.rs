use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
    }

    let result = match a * b % 2 {
        0 => "Even",
        _ => "Odd",
    };

    println!("{result}");
}
