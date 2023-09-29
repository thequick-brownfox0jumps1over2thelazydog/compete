#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: isize,
        Y: isize,
    }

    const NOT_FOUND: &str = "-1 -1 -1";
    const MAN: isize = 10000;
    const GOSEN: isize = 5000;
    const SEN: isize = 1000;

    if MAN * N < Y {
        println!("{NOT_FOUND}");
        return;
    }

    for i in (0..=(Y / MAN)).rev() {
        let man = i * MAN;
        if (N - i) * GOSEN < Y - man {
            break;
        }
        if (N - i) * SEN > Y - man {
            continue;
        }

        for j in (0..=(Y - man) / GOSEN).rev() {
            let k = N - i - j;
            if man + j * GOSEN + k * SEN == Y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("{NOT_FOUND}");
}
