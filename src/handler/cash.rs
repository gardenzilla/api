use std::convert::{TryFrom, TryInto};

use crate::{prelude::*, services::Services};
use gzlib::proto::cash::{
  new_transaction::CartId, BulkRequest, ByIdRequest, DateRangeRequest, NewTransaction,
  TransactionObject,
};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct DateRangeForm {
  date_from: String, // RFC3339
  date_till: String, // RFC3339
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionForm {
  transaction_id: String,
  cart_id: Option<u32>,
  amount: i32,
  reference: String,
  comment: String,
  created_by: u32,
  created_at: String,
}

impl TryFrom<TransactionObject> for TransactionForm {
  type Error = ApiError;

  fn try_from(to: TransactionObject) -> Result<Self, Self::Error> {
    let res = Self {
      transaction_id: to.transaction_id,
      cart_id: match to.cart_id.ok_or(ApiError::internal_error(
        "Tranzakció konverziós hiba. Nincs cart id object",
      ))? {
        gzlib::proto::cash::transaction_object::CartId::Cart(cid) => Some(cid),
        gzlib::proto::cash::transaction_object::CartId::None(_) => None,
      },
      amount: to.amount,
      reference: to.reference,
      comment: to.comment,
      created_by: to.created_by,
      created_at: to.created_at,
    };
    Ok(res)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTransactionPurchaseForm {
  cart_id: u32,
  amount: i32,
  reference: String,
  comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewTransactionGeneralForm {
  amount: i32,
  reference: String,
  comment: String,
}

pub async fn new_transaction_purchase(
  uid: u32,
  mut services: Services,
  nt: NewTransactionPurchaseForm,
) -> ApiResult {
  let transaction: TransactionForm = services
    .cash
    .create_transaction(NewTransaction {
      amount: nt.amount,
      reference: nt.reference,
      comment: nt.comment,
      created_by: uid,
      cart_id: Some(CartId::Cart(nt.cart_id)),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&transaction))
}

pub async fn new_transaction_general(
  uid: u32,
  mut services: Services,
  nt: NewTransactionGeneralForm,
) -> ApiResult {
  let transaction: TransactionForm = services
    .cash
    .create_transaction(NewTransaction {
      amount: nt.amount,
      reference: nt.reference,
      comment: nt.comment,
      created_by: uid,
      cart_id: Some(CartId::None(())),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&transaction))
}

pub async fn get_by_id(transaction_id: String, _uid: u32, mut services: Services) -> ApiResult {
  let transaction: TransactionForm = services
    .cash
    .get_by_id(ByIdRequest { transaction_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&transaction))
}

pub async fn get_bulk(
  _uid: u32,
  mut services: Services,
  transaction_ids: Vec<String>,
) -> ApiResult {
  let mut all = services
    .cash
    .get_bulk(BulkRequest { transaction_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<TransactionForm> = Vec::new();
  while let Some(transaction) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(transaction.try_into()?);
  }
  Ok(reply::json(&result))
}

pub async fn get_balance(_uid: u32, mut services: Services) -> ApiResult {
  let balance: i32 = services
    .cash
    .get_balance(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .balance;
  Ok(reply::json(&balance))
}

pub async fn get_by_date_range(
  _uid: u32,
  mut services: Services,
  date_range: DateRangeForm,
) -> ApiResult {
  let transaction_ids = services
    .cash
    .get_by_date_range(DateRangeRequest {
      date_from: date_range.date_from,
      date_till: date_range.date_till,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .transaction_ids;

  Ok(warp::reply::json(&transaction_ids))
}
