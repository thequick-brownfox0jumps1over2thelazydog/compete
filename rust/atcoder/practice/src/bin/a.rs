use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        s: String,
    }

    println!("{} {}", a + b + c, s);
}
