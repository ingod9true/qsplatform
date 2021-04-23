

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct User {
    username: String,
    user_age: u8,
    online: bool,
    tel:Option<String>,
    height: f32,
    hist_names:Vec<String>,

}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Job {
    company: Option<String>,
    pay: u32,
    work_on:bool
}

impl fmt::Display for Job {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "< {:?},{:?},{:?} >", self.company,self.pay,self.work_on)
    }
}


#[test]
fn textdemo(){

    let st:String = String::from(r#"
    {
    "id": 2,
    "sang":"zhang",
    "company": "google",
    "pay": 123,
    "work_on":true
    }
    "#);
    let deserialized: Job = serde_json::from_str(&st).unwrap();
    println!("{}",deserialized);

}


// rbatis ??
