use serde::{Serialize, Deserialize};
use serenity::model::prelude::{UserId};
use std::fs;
use std::path::Path;

#[derive(PartialOrd, PartialEq, Debug)]
#[derive (Serialize, Deserialize)]
pub enum TaskStatus {
    Waiting,
    Doing,
    Complete,
    Cancelled,
}

#[derive(Serialize, Deserialize)]
pub struct Task{
    pub name : String,
    pub description : String,
    pub status : TaskStatus,
}

#[derive(Serialize, Deserialize)]
pub struct UserData{
    pub tasks : Vec<Task>,
    pub command_prefix : String,
}

impl UserData{
    pub fn new() -> UserData {
        UserData{
            command_prefix : ">".to_string(),
            tasks : Vec::new(),
        }
    }

    pub fn add_task(&mut self, task : Task){
        self.tasks.push(task);
    }

    pub fn get_tasks(&self) -> &Vec<Task>{
        &self.tasks
    }
}

impl Task{
    pub fn new(name : String, description : String) -> Task{
        Task{
            name : name,
            description : description,
            status : TaskStatus::Waiting,
        }
    }
}

pub fn init_if_not_exist(id:&UserId) {
    let path = format!("data/userdata_{}.json", id.0);
    if !Path::new(&path).exists() {
        let userdata = UserData::new();
        save(&userdata, &id);
    }
}

pub fn load(id:&UserId) -> UserData {
    let path = format!("data/userdata_{}.json", id.0);
    let data = fs::read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn save(data: &UserData, id: &UserId) {
    let path = format!("data/userdata_{}.json", id.0);
    let data = serde_json::to_string(&data).unwrap();
    fs::write(path, data).unwrap();
}