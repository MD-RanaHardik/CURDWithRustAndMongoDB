use mongodb::{
    bson::{doc, Document, oid::ObjectId, from_document, self, Timestamp,},
    Client, Collection,
};
mod utility;
use serde::{Deserialize, Serialize};
use tokio::task;
use chrono::prelude::*;




#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    _id:ObjectId,
    Username:String,
    Password:String,
    Employee_name: String,
    Employee_salary: u64,
    Employee_designation: String,
    Office_in_out_time :Vec<Office_in_out_time>,
    Leves:Vec<Leves>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Office_in_out_time{
    Date:String,
    In_Time:String,
    Out_Time:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Leves{
    Applied_date:String,
    Leave_applied_for_date:String,
    Reason:String,
    Is_approved:bool

}

#[tokio::main]
async fn main() {

    let mut is_read_choice = true;
    
    let client = utility::connect_to_mongo().await;
    let col:Collection<Employee> = utility::get_collection(client);


    while is_read_choice {
        println!("\n\n 1. For add new employee \n 2. For fetch data of all employee \n 3. For update details \n 4. For delete employee \n 5. For increse salary of employee \n 6. For decrese salary of employee");
        let choice = utility::read_input("yor choice".to_string());
        match choice.as_str() {
            "1" => {
               
                utility::add_new_employee(col.clone()).await;
            }
            "2" => {
                
                utility::get_all_employee_details(col.clone()).await;
            }
            "3" => {
            
                utility::update_employee_details(col.clone()).await;
            }
            "4" => {
                
                utility::delete_employee(col.clone()).await;
            },
            "5"=>{
                
                utility::increse_salary(col.clone()).await;
            }
            "6"=>{
                
                utility::decrese_salary(col.clone()).await;
            }
            "7"=>{
            
                utility::start_timer(col.clone()).await;
            }
            _ => {}
        }
    }
}
