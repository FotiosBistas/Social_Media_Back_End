use std::io;
mod config;
mod profile;
mod kd_client;

fn main() {


    loop{
        println!("Options:");

        println!("0.Exit");
        println!("1.Sign up");
        println!("2.Log in");

        let mut option = String::new();

        io::stdin().read_line(&mut option).unwrap();
        let option:u8 = match option.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("You gave an erroneous input");
                continue;
            }
        };

        match option {
            1 => {
                kd_client::operations::signup(&prof);
            },
            2 => {
                kd_client::operations::login(&prof);
            },
            _ | 0 => {break;}

        }

    }
}


