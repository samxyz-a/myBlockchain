#![allow(unused_imports)]

use sha2;
use serde;
use serde_json;
use chrono;



pub struct Block{
 pub previous_hash:String,
 pub hash:String,
 pub data:String,
 pub index:u32,
 pub timestamp:i64,
}





fn main(){
    println!("Coding my own blockchain in rust from scratch");
}
