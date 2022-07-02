mod profile; 



fn main() {
    let prof = profile::Profile::new("Fotis","1234");
    println!("{}",prof);
}


fn return_int() -> i32{
    432 
}