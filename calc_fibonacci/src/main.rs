fn main() {
    println!("total {:?} = {}", arrs,total)
}
fn fi(n:u32) -> u32 {
    if n == 0 {
        return 0
    }
    if n == 1 || n == 2 {
        return 1
    }
    fi(n-1) + fi(n-2)
     
}
 