#![allow(unused_imports)]

use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use serde_json;
use chrono;
use hex;

#[derive(Serialize, Deserialize)]
pub struct Block{
 pub previous_hash:String,

 #[serde(skip)]
 pub hash:String,

 pub data:String,
 pub index:u32,
 pub timestamp:i64,
}


fn hash(b:Block){
    let data=serde_json::to_string(&b).unwrap();
    let mut hasher =Sha256::new();
    hasher.update(data);

    let hash=hasher.finalize();
    //let hash_string = format!("{:x}", hash);
    //b.hash = format!("{:x}", hash);
    let result=hex::encode(hash);
    println!("{:x}",result);
    b.hash=result;
}


fn main(){
    println!("Coding my own blockchain in rust from scratch");
}
