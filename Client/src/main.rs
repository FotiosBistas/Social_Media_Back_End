
mod profile;
mod kd_client;

fn main() {
    let prof = profile::Profile::new("Fotis","1234");
    println!("{}",prof);
    kd_client::operations::login(&prof);
}


