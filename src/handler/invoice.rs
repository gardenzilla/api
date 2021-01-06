use crate::prelude::*;
use crate::UserId;
use gzlib::proto;
use proto::invoice::invoice_client::*;
use protos;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceResponse {
  id: String,
  purchase_id: String,
  invoice_id: String,
  related_storno: String,
  created_by: String,
  created_at: String,
}

impl From<proto::invoice::InvoiceData> for InvoiceResponse {
  fn from(f: proto::invoice::InvoiceData) -> Self {
    InvoiceResponse {
      id: f.id,
      purchase_id: f.purchase_id,
      invoice_id: f.invoice_id,
      related_storno: f.related_storno,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _InvoiceForm {
  purchase_id: String,
  customer: Customer,
  items: Vec<Item>,
  payment_kind: String,
  payment_duedate: String,
  date: String,
  completion_date: String,
  total_net: i32,
  total_vat: i32,
  total_gross: i32,
  created_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
  id: String,
  name: String,
  tax_number: String,
  zip: String,
  location: String,
  street: String,
  email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
  name: String,
  quantity: i32,
  unit: String,
  price_unit_net: i32,
  vat: String,
  total_price_net: i32,
  total_price_vat: i32,
  total_price_gross: i32,
  comment: String,
}

impl From<_InvoiceForm> for proto::invoice::InvoiceForm {
  fn from(f: _InvoiceForm) -> Self {
    proto::invoice::InvoiceForm {
      purchase_id: f.purchase_id,
      customer: Some(proto::invoice::invoice_form::Customer {
        id: f.customer.id,
        name: f.customer.name,
        tax_number: f.customer.tax_number,
        zip: f.customer.zip,
        location: f.customer.location,
        street: f.customer.street,
        email: f.customer.email,
      }),
      items: f
        .items
        .iter()
        .map(|i| proto::invoice::invoice_form::Item {
          name: i.name.clone(),
          quantity: i.quantity.clone(),
          unit: i.unit.clone(),
          price_unit_net: i.price_unit_net,
          vat: i.vat.clone(),
          total_price_net: i.total_price_net,
          total_price_vat: i.total_price_vat,
          total_price_gross: i.total_price_gross,
          comment: i.comment.clone(),
        })
        .collect::<Vec<proto::invoice::invoice_form::Item>>(),
      payment_kind: f.payment_kind,
      payment_duedate: f.payment_duedate,
      date: f.date,
      completion_date: f.completion_date,
      total_net: f.total_net,
      total_vat: f.total_vat,
      total_gross: f.total_gross,
      created_by: f.created_by,
    }
  }
}

pub async fn new_invoice(
  userid: UserId,
  mut client: InvoiceClient<Channel>,
  new_invoice_form: _InvoiceForm,
) -> ApiResult {
  let response = client
    .create_new(proto::invoice::InvoiceForm::from(new_invoice_form))
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let res = match response.invoice {
    Some(r) => InvoiceResponse::from(r),
    None => {
      return Err(ApiError::internal_error("A válasz nem tartalmaz számla objektumot").into())
    }
  };

  Ok(reply::json(&res))
}
