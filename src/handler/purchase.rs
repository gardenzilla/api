use std::convert::{TryFrom, TryInto};

use crate::{prelude::*, services::Services};
use gzlib::proto::{self, purchase::PurchaseInfoObject};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseInfoForm {
  purchase_id: String,
  customer: Option<CustomerForm>,
  upl_count: u32,
  total_net_price: u32,
  total_vat: u32,
  total_gross_price: u32,
  balance: i32,
  payable: i32,
  document_invoice: bool,
  date_completion: String,
  payment_duedate: String,
  payment_expired: bool,
  profit_net: i32,
  restored: bool,
  created_by: u32,
  created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerForm {
  id: u32,
  name: String,
  zip: String,
  location: String,
  street: String,
}

impl From<PurchaseInfoObject> for PurchaseInfoForm {
  fn from(f: PurchaseInfoObject) -> Self {
    Self {
      purchase_id: f.purchase_id,
      customer: match f.customer {
        Some(c) => Some(CustomerForm {
          id: c.customer_id,
          name: c.name.clone(),
          zip: c.zip.clone(),
          location: c.location.clone(),
          street: c.street.clone(),
        }),
        None => None,
      },
      upl_count: f.upl_count,
      total_net_price: f.total_net_price,
      total_vat: f.total_vat,
      total_gross_price: f.total_gross_price,
      balance: f.balance,
      payable: f.payable,
      document_invoice: f.document_invoice,
      date_completion: f.date_completion,
      payment_duedate: f.payment_duedate,
      payment_expired: f.payment_expired,
      profit_net: f.profit_net,
      restored: f.restored,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

pub async fn purchase_get_all(_uid: u32, mut services: Services) -> ApiResult {
  let res: Vec<String> = services
    .purchase
    .purchase_get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .purchase_ids;
  Ok(reply::json(&res))
}

pub async fn get_bulk(_uid: u32, mut services: Services, purchase_ids: Vec<String>) -> ApiResult {
  let mut all = services
    .purchase
    .purchase_get_info_bulk(proto::purchase::PurchaseBulkRequest { purchase_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<PurchaseInfoForm> = Vec::new();
  while let Some(pinfo) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(pinfo.into());
  }
  Ok(reply::json(&result))
}
