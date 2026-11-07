use std::hint::black_box;

#[inline(never)]
fn sum_to_n(n: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut i: i32 = 1;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}

fn main() {
    let n = black_box(10_i32);
    println!("{}", sum_to_n(n));
}