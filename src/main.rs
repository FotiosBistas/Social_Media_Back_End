mod profile; 
mod KDclient; 


fn main() {
    let prof = profile::Profile::new("Fotis","1234");
    println!("{}",prof);
    KDclient::operations::login(&prof)
}


