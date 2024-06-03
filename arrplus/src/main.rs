use std::io;
fn main() {
    println!("请输入五个整数，逗号分割");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("faild to read");
    let mut arrs = [0u32;5];
    let mut index = 0;

    for n in s.trim().split(",") {
        let nn = n.trim().parse().expect("please input a correct value");
        arrs[index] = nn;
        index +=1;

        if index == 5 {
            break;
        }
    }

    let total = arr_plus(&arrs[..]);
    println!("total {:?} = {}", arrs,total)
}

fn arr_plus(i:&[u32]) -> u32 {
    let mut total = 0u32;
    for n in i {
        total += n;
    }
    total
}
