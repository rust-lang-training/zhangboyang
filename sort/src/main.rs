// use std::{cmp::Ordering};
use rand::Rng;
fn main() {
    println!("随即生成10个数值");
    let mut arr = [0u32;10];
    for i in 0..10 {
        arr[i] = rand::thread_rng().gen_range(1..100);
    }
    println!("{:?}",arr);

    let len = arr.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j,j+1)
            }
        }
    }
    println!("after:{:?}",arr);
}
