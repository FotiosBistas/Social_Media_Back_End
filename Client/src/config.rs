pub struct Config{
    username: String,
    password: String,
}


impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &str>{
        if args.len() < 2 {
            panic!("Less than two arguments received");
        }

        args.next();

        let username = match args.next(){
            None => return Err("Could not get username from command line"),
            Some(str) => str
        };

        let password = match args.next() {
            None => return Err("Could not get password from command line"),
            Some(str) => str
        };

        Ok(Config{username,password})
    }
}