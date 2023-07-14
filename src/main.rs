use rand::Rng;
use std::io::{self, Write};

fn main() {
    //program simple random dice(dadu)
    loop {
        println!("\n===========TEBAK ANGKA==========\n");

        print!("acak dadu y/n: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        //koversi small case
        let input = input.trim().to_lowercase();

        if input == "y" {
            let input_user: u8;

            loop {
                //input guess(tebakan)user
                let mut input_user_str = String::new();
                print!("masukan tebakan mu 1 sampai 6: ");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input_user_str).unwrap();

                input_user = match input_user_str.trim().parse() {
                    Ok(num) => {
                        if num >= 1 && num <= 6 {
                            num
                        } else {
                            println!("inputan tikak valid");
                            continue;
                        }
                    }
                    _ => {
                        println!("inputan tidak valid masukan angka");
                        continue;
                    }
                };
                break;
            }
            //random dice(dadu)
            let dice = rand::thread_rng().gen_range(1..=6);

            println!("================================\n");
            println!("dadu: {}", dice);
            println!("you: {}", input_user);

            //win or lose
            if input_user > dice {
                println!("dadu lebih kecil tebakanmu belum tepat!!!");
            } else if input_user == dice {
                println!("selamat tebakan mu benar!!!");
            } else {
                println!("dadu lebih besar tebakanmu belum tepat!!!");
            }
        } else if input == "n" {
            break;
        } else {
            println!("inputan salah pilih y or n !!!!");
        }
    }
}
