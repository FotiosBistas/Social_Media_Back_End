use std::collections::VecDeque;
use std::fs::File;
use std::ops::Index;
use std::sync::{Arc, Mutex};

pub struct Server<'a>{
    //active connections in the server
    catalog: Arc<Mutex<Vec<(String,String)>>>,
    //an array showing what client id corresponds to what index in the catalog
    indexes: Vec<u32>,
    //queue for accessing a file contains the id of the client that tries to access
    //instead of using a hash map there will be a second data structure containing
    //the actively used files
    file_priority_queue: Arc<Mutex<VecDeque<(u32,u32)>>>,
    active_files: Arc<Mutex<Vec<&'a File>>>,
}


impl<'a> Server<'a>{
    pub fn new() -> Server<'a>{
        Server{
            catalog: Arc::new(Mutex::new(Vec::with_capacity(10))),
            indexes: Vec::with_capacity(10),
            file_priority_queue: Arc::new(Mutex::new(VecDeque::new())),
            active_files: Arc::new(Mutex::new(Vec::new())),
        }
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

    pub fn add_to_priority_queue(&mut self,client_id:u32,file: &'a File){

        let mut active_files = self.active_files.lock().unwrap();
        let mut active_files = &mut *active_files;
        let index_of_file = active_files.iter().position(|x| std::ptr::eq(x,&file));

        let mut file_priority_queue = self.file_priority_queue.lock().unwrap();
        let mut file_priority_queue = &mut *file_priority_queue;

        if index_of_file != None{
            file_priority_queue.push_front((client_id,index_of_file.unwrap() as u32));
        }else{
            println!("inserted file");
            active_files.push(file);
            file_priority_queue.push_front((client_id,index_of_file.unwrap() as u32));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;




    #[test]
    fn file_is_not_contained_and_will_be_added(){
        let mut server = Server::new();
        let file = File::open("SocialGraph.txt").unwrap();
        server.add_to_priority_queue(2,&file);

        let mut active_files = server.active_files.lock().unwrap();
        //let mut file_priority_queue = server.file_priority_queue.lock().unwrap();
        let active_files = &mut *active_files;
        assert_eq!(active_files.len(), 1)
    }

    #[test]
    fn file_is_contained_and_client_will_be_added_to_queue(){

    }
}