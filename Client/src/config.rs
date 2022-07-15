pub struct Config{
    pub username: String,
    pub password: String,
}


impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{

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