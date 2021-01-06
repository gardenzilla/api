use crate::prelude::*;
use crate::UserId;
use protos;
use protos::customer::customer_client::CustomerClient;
use protos::customer::*;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
  zip: String,
  location: String,
  address: String,
}

#[derive(Serialize, Deserialize)]
pub struct Customer {
  id: String,
  name: String,
  email: String,
  phone: String,
  tax_number: String,
  address: Address,
  has_user: bool,
  users: Vec<String>,
  date_created: String,
  created_by: String,
}

impl From<&CustomerObj> for Customer {
  fn from(c: &CustomerObj) -> Self {
    let address = if let Some(addr) = &c.address {
      Address {
        zip: addr.zip.to_owned(),
        location: addr.location.to_owned(),
        address: addr.address.to_owned(),
      }
    } else {
      Address {
        zip: "".into(),
        location: "".into(),
        address: "".into(),
      }
    };
    Self {
      id: c.id.to_owned(),
      name: c.name.to_owned(),
      email: c.email.to_owned(),
      phone: c.phone.to_owned(),
      tax_number: c.tax_number.to_owned(),
      address: address,
      has_user: c.has_user,
      users: c.users.to_owned(),
      date_created: c.date_created.to_owned(),
      created_by: c.created_by.to_owned(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerNew {
  name: String,
  email: String,
  phone: String,
  tax_number: String,
  zip: String,
  location: String,
  address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerUpdateForm {
  name: String,
  email: String,
  phone: String,
  tax_number: String,
  zip: String,
  location: String,
  address: String,
}

impl CustomerNew {
  fn to_request(self, created_by: UserId) -> CreateNewRequest {
    CreateNewRequest {
      name: self.name,
      email: self.email,
      phone: self.phone,
      tax_number: self.tax_number,
      zip: self.zip,
      location: self.location,
      address: self.address,
      created_by: created_by.to_string(),
    }
  }
}

pub async fn create_new(
  userid: UserId,
  mut client: CustomerClient<Channel>,
  customer_object: CustomerNew,
) -> ApiResult {
  let customer = client
    .create_new(customer_object.to_request(userid))
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  if let Some(customer) = customer.customer {
    let _user: Customer = (&customer).into();
    return Ok(reply::json(&_user));
  }
  Err(ApiError::not_found().into())
}

pub async fn get_all(_: UserId, mut client: CustomerClient<Channel>) -> ApiResult {
  let all = client
    .get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  let v = all
    .customers
    .iter()
    .map(|u| u.into())
    .collect::<Vec<Customer>>();
  Ok(warp::reply::json(&v))
}

pub async fn get_by_id(
  id: String,
  _userid: UserId,
  mut client: CustomerClient<Channel>,
) -> ApiResult {
  let customer = client
    .get_by_id(GetByIdRequest { customer_id: id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  if let Some(customer) = customer.customer {
    let _user: Customer = (&customer).into();
    return Ok(reply::json(&_user));
  }
  Err(ApiError::not_found().into())
}

pub async fn update(
  customer_id: String,
  _: UserId,
  mut client: CustomerClient<Channel>,
  customer_form: CustomerUpdateForm,
) -> ApiResult {
  let res = match client
    .update_by_id(UpdateByIdRequest {
      customer_id: customer_id.clone(),
      customer: Some(CustomerUpdateObj {
        id: customer_id,
        name: customer_form.name,
        email: customer_form.email,
        phone: customer_form.phone,
        tax_number: customer_form.tax_number,
        address: Some(protos::customer::Address {
          zip: customer_form.zip,
          location: customer_form.location,
          address: customer_form.address,
        }),
      }),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .customer
  {
    Some(c) => c,
    None => return Err(ApiError::internal_error("Not customer object in update response").into()),
  };
  let customer: Customer = (&res).into();
  Ok(warp::reply::json(&customer))
}
