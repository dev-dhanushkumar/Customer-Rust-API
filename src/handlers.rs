use std::convert::Infallible;

use warp::{self,http::StatusCode};

use crate::db::Db;
use crate::models::Customer;

//Return a list of customer as JSON

pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await;
    let customers: Vec<Customer> = customers.clone();
    Ok(warp::reply::json(&customers))
}

///Create a new customers
/// 
///Adds a new customer object to the data store if the customer
/// doesn't already exist
/// 
/// #Argument
/// 
/// * `new_customer`- `Customer` type
/// * `db` - `Db` -> thread safe vector of Customer objects
pub async fn create_customer(
    new_customer:Customer,
    db:Db,
) ->Result<impl warp::Reply, Infallible>{
    let mut customers=db.lock().await;

    for customer in customers.iter(){
        if customer.guid==new_customer.guid{
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    customers.push(new_customer);

    Ok(StatusCode::CREATED)
}

///Get a single customer from the data store
/// 
/// Return a JSON object of an existing sustomer.If the customer
/// is not found , It return a NOT FOUND staus code
/// #Argument
/// 
/// * `guid` - String -> the id of the customer to retrieve
/// * `db` - `Db` ->the thread safe data store
pub async fn get_customer(guid:String, db:Db)-> Result<Box<dyn warp::Reply>, Infallible>{
    let customers=db.lock().await;

    for customer in customers.iter(){
        if customer.guid ==guid{
            return Ok(Box::new(warp::reply::json(&customer)));
        }
    }

    Ok(Box::new(StatusCode::NOT_FOUND))
}

///Update an existing customer
/// 
/// Overwrites an existing customer in the data store abd return
/// an OK status code. If the customer is not found, a NOT_FOUND status
/// code is return
/// 
/// #Argument
/// 
/// * `update_customer` - `Customer` -> updated customer info
/// * `db` - `Db` -> thread safe data store
pub async fn update_customer(
    guid:String,
    updated_customer:Customer,
    db:Db,
) ->Result<impl warp::Reply,Infallible>{
    let mut customers=db.lock().await;

    for customer in customers.iter_mut(){
        if customer.guid==guid{
            *customer=updated_customer;
            return  Ok(StatusCode::OK);
        }
    }

    Ok(StatusCode::NOT_FOUND)
}

///DDelete a customer from the data store
/// 
/// If the customer exixts in the data store, the customer
/// removed and a NO CONTENT status code is returned. If the customer
/// doesn't exists , a NOT FOUND status code  is returned
/// 
///#Argument
/// 
/// * `guid` - String -> the id of the customer to delete
/// * `db` -`Db` -> thread safe data store
pub async fn delete_customer(guid:String,db:Db) ->Result<impl warp::Reply,Infallible>{
    let mut customers=db.lock().await;

    let customer_count=customers.len();

    customers.retain(|customer| customer.guid !=guid);

    let deleted=customers.len() !=customer_count;
    if deleted{
        Ok(StatusCode::NO_CONTENT)
    }else{
        Ok(StatusCode::NO_CONTENT)
    }
}