use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::Index;
use std::sync::{Arc, Mutex};

pub struct Server<'a>{
    //active connections in the server
    catalog: Arc<Mutex<Vec<(String,String,u32)>>>,
    //active files are all the files that are opened and are being modified or read in the server 
    active_files: Arc<Mutex<Vec<FileWrapper<'a>>>>,
}

///A wrapper struct containing the clients wanting to access the specific file. The data structure is a FIFO queue. 
struct FileWrapper<'a>{
    file: &'a File, 
    client_queue: VecDeque<u32> 
}

impl<'a> FileWrapper<'a>{
    fn new(file:&'a File) -> FileWrapper<'a>{
        FileWrapper { file: file, client_queue: VecDeque::with_capacity(8) }
    }
}

///Helper trait used for comparing two files. Reduces boiler plate code. 
trait EqualFile{
    fn is_equal(&self,other: &File) -> Result<bool,std::io::Error>; 
}


impl EqualFile for File{
    fn is_equal(&self,other: &File) -> Result<bool,std::io::Error> {

        if other.metadata()?.len() != self.metadata()?.len() {
            return Ok(false);
        }

        let f1 = BufReader::new(self); 
        let f2 = BufReader::new(other); 

        for (b1,b2) in f1.bytes().zip(f2.bytes()){
            if b1? != b2? {
                return Ok(false); 
            }
        }
        
        return Ok(true); 
    }
}

impl<'a> Server<'a>{
    pub fn new() -> Server<'a>{
        Server{
            catalog: Arc::new(Mutex::new(Vec::with_capacity(10))),
            active_files: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_new_catalog_entry(&mut self,tuple: (String,String,u32)){
        let mut entry = self.catalog.lock().unwrap();


        let mut catalog = &mut *entry;

        //check if catalog contains the entry
        for i in catalog.iter(){
            if *i.0 == tuple.0 && *i.1 == tuple.1 && i.2 == tuple.2{
                return;
            }
        }
        catalog.push(tuple);
    }

    ///Adds a file to the active queue and returns the index of the file added. 
    pub fn add_file_to_active_files(&mut self,file: &'a File){
        let mut active_files = self.active_files.lock().unwrap();
        let mut active_files = &mut *active_files;
        let index_of_file = active_files.iter().position(|x| x.file.is_equal(file).unwrap());

        if index_of_file == None {
            active_files.push(
                FileWrapper::new(file)
            ); 
        }
    }

    pub fn remove_file_from_active_files(&mut self,file: &'a File){
        let mut active_files = self.active_files.lock().unwrap();
        let mut active_files = &mut *active_files;
        let index_of_file = active_files.iter().position(|x| x.file.is_equal(file).unwrap());
        if index_of_file != None {
            active_files.remove(index_of_file.unwrap()); 
        }
    }

    pub fn add_client_to_priority_queue(&mut self,client_id:u32,file: &'a File){
        let mut active_files = self.active_files.lock().unwrap(); 
        let mut active_files = &mut *active_files; 

        let index_of_file = active_files.iter().position(|x| x.file.is_equal(file).unwrap()); 
        if index_of_file != None{
            let mut file_wrapper = active_files.get_mut(index_of_file.unwrap()).unwrap(); 
            match file_wrapper.client_queue.iter().position(|x| *x == client_id){
                None => file_wrapper.client_queue.push_back(client_id), 
                Some(_) => return, 
            }
        } 
    }

    pub fn remove_client_from_priority_queue(&mut self,file: &'a File){
        let mut active_files = self.active_files.lock().unwrap(); 
        let mut active_files = &mut *active_files; 

        let index_of_file = active_files.iter().position(|x| x.file.is_equal(file).unwrap()); 
        if index_of_file != None{
            let mut file_wrapper = active_files.get_mut(index_of_file.unwrap()).unwrap(); 
            file_wrapper.client_queue.pop_front();
        }
    }

}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;


    #[test]
    fn new_catalog_entry_is_added(){
        let mut server = Server::new(); 

        server.add_new_catalog_entry((String::from("192.168.1.5"),String::from("8080"),2)); 
        let catalog = &*server.catalog.lock().unwrap(); 
        assert_eq!(catalog.len(),1); 
    }

    #[test]
    fn already_existing_catalog_entry_is_not_added(){
        let mut server = Server::new(); 

        server.add_new_catalog_entry((String::from("192.168.1.5"),String::from("8080"),2)); 
        server.add_new_catalog_entry((String::from("192.168.1.5"),String::from("8080"),2)); 
        server.add_new_catalog_entry((String::from("192.168.1.5"),String::from("8080"),3)); 

        let catalog = &*server.catalog.lock().unwrap(); 
        assert_eq!(catalog.len(),2); 
    }

    #[test]
    fn file_is_removed_from_active_files(){
        let mut server = Server::new();

        let file = match File::open("C:/Users/fotis/GitHub/Social_Media_Back_End/server/SocialGraph.txt"){
            Ok(file) => file,
            Err(err) => panic!("Could not open file: {}",err),
        };

        server.add_file_to_active_files(&file);
        server.remove_file_from_active_files(&file); 

        let mut active_files = server.active_files.lock().unwrap(); 
        assert_eq!(active_files.len(),0);
    }

    #[test]
    fn client_is_removed_from_queue(){
        let mut server = Server::new();

        let file = match File::open("C:/Users/fotis/GitHub/Social_Media_Back_End/server/SocialGraph.txt"){
            Ok(file) => file,
            Err(err) => panic!("Could not open file: {}",err),
        };

        server.add_file_to_active_files(&file);
        server.add_client_to_priority_queue(2, &file); 
        server.add_client_to_priority_queue(3, &file); 
        server.add_client_to_priority_queue(4, &file);
    }

    #[test]
    fn file_is_not_contained_and_will_be_added(){
        let mut server = Server::new();

        let file = match File::open("C:/Users/fotis/GitHub/Social_Media_Back_End/server/SocialGraph.txt"){
            Ok(file) => file,
            Err(err) => panic!("Could not open file: {}",err),
        };

        server.add_file_to_active_files(&file); 

        let mut active_files = server.active_files.lock().unwrap(); 
        let active_files = &mut *active_files;

        assert_eq!(active_files.len(),1);
    }


    #[test]
    fn when_file_is_opened_twice_the_active_file_queue_stays_the_same(){
        let mut server = Server::new(); 

        let file = match File::open("C:/Users/fotis/GitHub/Social_Media_Back_End/server/SocialGraph.txt"){
            Ok(file) => file,
            Err(err) => panic!("Could not open file: {}",err),
        }; 
        server.add_file_to_active_files(&file); 

        let file = match File::open("C:/Users/fotis/GitHub/Social_Media_Back_End/server/SocialGraph.txt"){
            Ok(file) => file,
            Err(err) => panic!("Could not open file: {}",err),
        };  
        server.add_file_to_active_files(&file);

        let mut active_files = server.active_files.lock().unwrap(); 
        let active_files = &mut *active_files;

        assert_eq!(active_files.len(),1);
    }

    #[test]
    fn file_is_contained_and_client_will_be_added_to_queue(){
        let mut server = Server::new(); 
        let file = match File::open("C:/Users/fotis/GitHub/Social_Media_Back_End/server/SocialGraph.txt"){
            Ok(file) => file,
            Err(err) => panic!("Could not open file: {}",err),
        };    

        server.add_file_to_active_files(&file); 
        server.add_client_to_priority_queue(2,&file);
        server.add_client_to_priority_queue(3,&file);

        let mut active_files = server.active_files.lock().unwrap();
        let active_files = &mut *active_files; 

        let index_of_file = active_files.iter().position(|x| x.file.is_equal(&file).unwrap()); 
        
        let file_wrapper = active_files.get(index_of_file.unwrap()).unwrap(); 
        

        assert_eq!(file_wrapper.client_queue.len(),2);
    }

    #[test]
    fn clientid_is_the_same_as_another_clientid_and_will_not_add_topriorityqueue(){
        let mut server = Server::new(); 
        let file = match File::open("C:/Users/fotis/GitHub/Social_Media_Back_End/server/SocialGraph.txt"){
            Ok(file) => file,
            Err(err) => panic!("Could not open file: {}",err),
        };        
        
        server.add_file_to_active_files(&file); 
        server.add_client_to_priority_queue(2,&file);
        server.add_client_to_priority_queue(2,&file);

        let mut active_files = server.active_files.lock().unwrap();
        let active_files = &mut *active_files; 

        let index_of_file = active_files.iter().position(|x| x.file.is_equal(&file).unwrap()); 
        
        let file_wrapper = active_files.get(index_of_file.unwrap()).unwrap(); 
        

        assert_eq!(file_wrapper.client_queue.len(),1);
    }
}