

use std::io::Lines;

pub fn search_in_file(needle:&str,lines:Vec<Lines>,ignore_case:bool){
   for line in lines{
       match ignore_case{
           true=>{
              if line.to_lowercase().contains(needle.to_lowercase()){
                   println!("{}",line);
               }
               
           }
           false=>{
               if line.contains(needle.to_lowercase()){
                   println!("{}",line);
               }
           }
       }
   }
}