use serde::{Serialize, Deserialize};
use serenity::model::prelude::{UserId};
use std::fs;
use std::path::Path;

#[derive (Serialize, Deserialize)]
pub enum TaskStatus {
    Waiting,
    Doing,
    Complete,
    Cancelled,
}

#[derive(Serialize, Deserialize)]
pub struct Task{
    name : String,
    description : String,
    status : TaskStatus,
}

#[derive(Serialize, Deserialize)]
pub struct UserData{
    pub tasks : Vec<Task>,
    pub command_prefix : String,
}

impl UserData{
    fn new() -> UserData {
        UserData{
            command_prefix : ">".to_string(),
            tasks : Vec::new(),
        }
    }

    fn save(&self, id:UserId) {
        let path = format!("data/userdata_{}.json", id.0);
        let data = serde_json::to_string(&self).unwrap();
        fs::write(path, data).unwrap();
    }
}

pub fn init_if_not_exist(id:&UserId) {
    let path = format!("data/userdata_{}.json", id.0);
    if !Path::new(&path).exists() {
        let userdata = UserData::new();
        userdata.save(*id);
    }
}

pub fn load(id:&UserId) -> UserData {
    let path = format!("data/userdata_{}.json", id.0);
    let data = fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}
