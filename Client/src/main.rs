extern crate core;

use std::{env, io, process};
use crate::config::Config;
use crate::profile::Profile;

mod config;
mod profile;
mod kd_client;



fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let prof = Profile::new(&config.username, &config.password);
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
                kd_client::operations::signup(&prof).unwrap_or_else(|err|{
                    eprintln!("An error occurred while trying to sign up: {}",err);
                });
            },
            2 => {
                kd_client::operations::login(&prof).unwrap_or_else(|err|{
                    eprintln!("An error occurred while trying to login: {}",err);
                });
            },
            _ | 0 => {break;}

        }

    }
}


