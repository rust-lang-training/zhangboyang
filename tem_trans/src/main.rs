use std::io;
// use rand::Rng;
// F=c * 1.8 + 32
// C=(F-32) / 1.8

fn main() {
    loop{

        println!("摄⽒度 --> 华⽒度请输⼊ 1");
        println!("华⽒度 --> 摄⽒度请输⼊ 2");
        println!("退出请输⼊ 0");
        println!("\n你的选择是：");
        
        let mut tem_type_choice = String::new();
      
        io::stdin().read_line(&mut tem_type_choice).expect("faild to read line");

        let tem_type_choice = tem_type_choice.trim();
        // println!("you guessed: {tem_type_choice}");
        if tem_type_choice == "0" {
            break;
        }
        if tem_type_choice == "1" {
            println!("\n please input f");
            let mut c = String::new();
            io::stdin().read_line(&mut c).expect("faild to read line");
            let c : f32 = c.trim().parse().expect("please input a correct value");
            let f = c_trans_f(c);
            println!("{:.2} 摄氏度 --> {:.2} 华氏度\n",c,f);
        }
    }
    
}
fn c_trans_f(c:f32) -> f32 {
    c * 1.8 + 32.0
}