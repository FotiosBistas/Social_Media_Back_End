
use std::sync::{Arc,Mutex};

pub struct Server{
    catalog :Arc<Mutex<Vec<(String,String)>>>
}


impl Server {
    pub fn new() -> Server{
        Server{ catalog: Arc::new(Mutex::new(Vec::with_capacity(10)))}
    }

    pub fn add_new_catalog_entry(&mut self,tuple: (String,String)){
        let mut entry = self.catalog.lock().unwrap();


        let mut catalog = &mut *entry;
        //check if catalog contains the entry
        for i in catalog.iter(){
            if *i.0 == tuple.0 && *i.1 == tuple.1{
                return;
            }
        }
        catalog.push(tuple);
    }
}