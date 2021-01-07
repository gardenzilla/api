use crate::{prelude::*, services::Services};
use gzlib::proto::customer::*;
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryForm {
  query: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerForm {
  id: u32,
  name: String,
  email: String,
  phone: String,
  tax_number: String,
  address_zip: String,
  address_location: String,
  address_street: String,
  date_created: String,
  created_by: u32,
}

impl From<CustomerObj> for CustomerForm {
  fn from(c: CustomerObj) -> Self {
    Self {
      id: c.id,
      name: c.name,
      email: c.email,
      phone: c.phone,
      tax_number: c.tax_number,
      address_zip: c.address_zip,
      address_location: c.address_location,
      address_street: c.address_street,
      date_created: c.date_created,
      created_by: c.created_by,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerNewForm {
  name: String,
  email: String,
  phone: String,
  tax_number: String,
  address_zip: String,
  address_location: String,
  address_street: String,
}

pub async fn create_new(uid: u32, mut services: Services, co: CustomerNewForm) -> ApiResult {
  let customer: CustomerForm = services
    .customer
    .create_new(NewCustomerObj {
      name: co.name,
      email: co.email,
      phone: co.phone,
      tax_number: co.tax_number,
      address_zip: co.address_zip,
      address_location: co.address_location,
      address_street: co.address_street,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&customer))
}

pub async fn get_all(_: u32, mut services: Services) -> ApiResult {
  let customer_ids: Vec<u32> = services
    .customer
    .get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .customer_ids;
  Ok(warp::reply::json(&customer_ids))
}

pub async fn get_by_id(customer_id: u32, _uid: u32, mut services: Services) -> ApiResult {
  let customer: CustomerForm = services
    .customer
    .get_by_id(GetByIdRequest { customer_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&customer))
}

pub async fn get_bulk(_uid: u32, mut services: Services, customer_ids: Vec<u32>) -> ApiResult {
  let mut all = services
    .customer
    .get_bulk(GetBulkRequest { customer_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<CustomerForm> = Vec::new();
  while let Some(customer) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(customer.into());
  }
  Ok(warp::reply::json(&result))
}

pub async fn update(_uid: u32, mut services: Services, cf: CustomerForm) -> ApiResult {
  let customer: CustomerForm = services
    .customer
    .update_by_id(CustomerObj {
      id: cf.id,
      name: cf.name,
      email: cf.email,
      phone: cf.phone,
      tax_number: cf.tax_number,
      address_zip: cf.address_zip,
      address_location: cf.address_location,
      address_street: cf.address_street,
      date_created: cf.date_created,
      created_by: cf.created_by,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(warp::reply::json(&customer))
}

pub async fn find(_uid: u32, mut services: Services, f: QueryForm) -> ApiResult {
  let customer_ids: Vec<u32> = services
    .customer
    .find_customer(FindCustomerRequest { query: f.query })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .customer_ids;
  Ok(reply::json(&customer_ids))
}
