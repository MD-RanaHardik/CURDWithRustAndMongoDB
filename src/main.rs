use mongodb::{
    bson::{doc, Document, oid::ObjectId, from_document,},
    Client, Collection,
};
mod utility;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    _id:ObjectId,
    Employee_name: String,
    Employee_salary: u64,
    Employee_designation: String,
}

#[tokio::main]
async fn main() {
    let mut is_read_choice = true;
    
    

    while is_read_choice {
        println!("\n\n 1. For add new employee \n 2. For fetch data of all employee \n 3. For update details \n 4. For delete employee");
        let choice = utility::read_input("yor choice".to_string());
        match choice.as_str() {
            "1" => {
                let client = utility::connect_to_mongo();

                let collection_selected = utility::get_collection(client.await);

                utility::add_new_employee(collection_selected.await).await;
            }
            "2" => {
                let client = utility::connect_to_mongo();

                let collection_selected = utility::get_collection(client.await);
                utility::get_all_employee_details(collection_selected.await).await;
            }
            "3" => {
                let client = utility::connect_to_mongo();

                let collection_selected = utility::get_collection(client.await);
                utility::update_employee_details(collection_selected.await).await;
            }
            "4" => {
                let client = utility::connect_to_mongo();

                let collection_selected = utility::get_collection(client.await);
                utility::delete_employee(collection_selected.await).await;
            }
            _ => {}
        }
    }
}
