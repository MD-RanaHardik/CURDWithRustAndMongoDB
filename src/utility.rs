use mongodb::{Client,bson::{doc, Document, oid::ObjectId},Collection,};
use std::io;
use futures::{stream::TryStreamExt, StreamExt};

use crate::Employee;


pub fn read_input(typeofinput:String)->String{

    let mut input =String::new();
    println!("Enter {}",typeofinput);
    io::stdin().read_line(&mut input);
    input.trim().to_string()
}

pub async fn connect_to_mongo()->Client{
    let client = Client::with_uri_str("mongodb://localhost:27017").await.expect("faild to connect with db");
    client
}


pub async fn get_collection(client:Client)->Collection<Employee>{
    
    let database = client.database("FirstDB");
    
 
    let collection_inst:Collection<Employee> =  database.collection::<Employee>("Employee");

    collection_inst
}


pub async fn add_new_employee(collection:Collection<Employee>){

    let emp_name = read_input("employee name".to_string());
    let emp_salary = read_input("employee salary".to_string());
    let emp_designation = read_input("employee designation".to_string());

    let docment_data:Employee = Employee {
        _id:ObjectId::new(),
        Employee_name : emp_name.to_string(),
        Employee_salary : emp_salary.parse::<u64>().unwrap(),
        Employee_designation :emp_designation
    };
    collection.insert_one(docment_data, None).await.expect("Faild to insert new employee");

}


pub async fn get_all_employee_details(collection:Collection<Employee>){     
    
    let mut result  = collection.find(None, None).await.expect("Faild to fetch data");

   while let Some(rs) = result.next().await {

       match rs {
        Ok(documentd)=>{
            print_output(format!("Employee name is {} Employee salary is {} and designation is {}",documentd.Employee_name,documentd.Employee_salary,documentd.Employee_designation).as_str(),"Employee details");
        },
        Err(e)=>{}
           
       }
   }
    
}

pub async fn delete_employee(collection:Collection<Employee>){
    let name  = read_input("employee name which you have to delete ".to_string());
    if name.is_empty(){
        print_output("Employee name can not be empty", "Error");
    }else{
        let filter  = doc! {"Employee_name":name};
        collection.delete_many(filter, None).await.expect("Faild to delete employeee");
        print_output("Employee successfully deleted", "Massage");
    }
    
}

pub async fn update_employee_details(collection:Collection<Employee>){
    let name  = read_input("employee name which you have to update ".to_string());

    let newname  = read_input("employee new name which you have to delete ".to_string());
    
    if name.is_empty(){
        print_output("Employee name can not be empty", "Error");
    }else{
        let filter  = doc! {"Employee_name":name};
        let update  = doc! {"$set":{"Employee_name":newname}};
        collection.update_one(filter, update,None).await.expect("Faild to delete employeee");
        print_output("Employee details successfully updated", "Massage");
    }
    
}


pub fn print_output(input: &str,msg:&str) {
    println!("{}",msg);
    if input.contains("\n") {
        let mut lines: Vec<&str> = input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
            .collect();

        lines.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

        println!(
            "+{}+",
            String::from("-").repeat(lines.last().unwrap().len() + 10)
        );

        for line in input
            .split("\n")
            .filter(|a| !a.is_empty())
            .map(|a| a.trim())
        {
            println!(
                "|{}{}{}|",
                String::from(" ").repeat(5),
                line,
                String::from(" ").repeat(lines.last().unwrap().len() - line.len() + 5)
            );
        }
        println!(
            "+{}+",
            String::from("-").repeat(lines.last().unwrap().len() + 10)
        );
    } else {
        println!("+{}+", String::from("-").repeat(input.len() + 10));
        println!(
            "|{}{}{}|",
            String::from(" ").repeat(2),
            input,
            String::from(" ").repeat(8)
        );
        println!("+{}+", String::from("-").repeat(input.len() + 10));
    }
}