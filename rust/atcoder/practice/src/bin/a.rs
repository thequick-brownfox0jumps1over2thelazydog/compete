use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
        c: u16,
        s: String,
    }

    println!("{} {}", a + b + c, s);
}
