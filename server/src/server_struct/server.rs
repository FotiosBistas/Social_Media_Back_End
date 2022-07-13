
pub struct Server{
    catalog: Vec<(String,u16)>
}


impl Server {
    pub fn new(){

    }

    pub fn add(&mut self, tuple:(String,u16)){
        self.catalog.push(tuple); 
    }
}