#![allow(dead_code)]
#![allow(unused_variables)]

use cargo_snippet::snippet;

#[snippet("constant")]
const UPPER_LARGE_PRIME: usize = 1e9 as usize + 7;

#[snippet("constant")]
const LOWER_LARGE_PRIME: usize = 998_244_353;
