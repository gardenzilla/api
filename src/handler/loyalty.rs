use std::convert::{TryFrom, TryInto};

use crate::{prelude::*, services::Services};
use gzlib::proto::loyalty::{
  transaction::TransactionKind, Account, Card, CardRequest, CustomerRequest, LoyaltyLevelRequest,
  NewAccount, QueryRequest, SetBirthdateRequest, Transaction, TransactionAllRequest,
};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionKindForm {
  Burn,
  Earn,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionForm {
  transaction_id: String,
  account_id: String,
  purchase_id: String,
  transaction_kind: TransactionKindForm,
  amount: i32,
  created_by: u32,
  created_at: String,
}

impl From<Transaction> for TransactionForm {
  fn from(f: Transaction) -> Self {
    let trkind: TransactionKind = TransactionKind::from_i32(f.transaction_kind).unwrap();
    Self {
      transaction_id: f.transaction_id,
      account_id: f.account_id,
      purchase_id: f.purchase_id,
      transaction_kind: match trkind {
        TransactionKind::Burn => TransactionKindForm::Burn,
        TransactionKind::Earn => TransactionKindForm::Earn,
      },
      amount: f.amount,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountForm {
  account_id: String,
  customer_id: u32,
  customer_birthdate: String,
  card_id: String,
  loyalty_level: String,
  balance_points: i32,
  yearly_gross_turnover: i32,
  created_at: String,
  created_by: u32,
}

impl From<Account> for AccountForm {
  fn from(f: Account) -> Self {
    Self {
      account_id: f.account_id,
      customer_id: f.customer_id,
      customer_birthdate: f.customer_birthdate,
      card_id: f.card_id,
      loyalty_level: f.loyalty_level,
      balance_points: f.balance_points,
      yearly_gross_turnover: f.yearly_gross_turnover,
      created_at: f.created_at,
      created_by: f.created_by,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewAccountForm {
  customer_id: u32,
  birthdate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetBirthdateForm {
  account_id: String,
  birthdate: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetLoyaltyLevelForm {
  account_id: String,
  loyalty_level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetCardForm {
  account_id: String,
  card_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryForm {
  customer_id: u32,
  birthdate: String,
}

pub async fn new_account(uid: u32, mut services: Services, f: NewAccountForm) -> ApiResult {
  let res: AccountForm = services
    .loyalty
    .create_account(NewAccount {
      customer_id: f.customer_id,
      birthdate: f.birthdate,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}

pub async fn get_by_customer_id(customer_id: u32, uid: u32, mut services: Services) -> ApiResult {
  let res: AccountForm = services
    .loyalty
    .get_account_by_customer_id(CustomerRequest { customer_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}

pub async fn get_by_card_id(card_id: String, uid: u32, mut services: Services) -> ApiResult {
  let res: AccountForm = services
    .loyalty
    .get_account_by_card_id(CardRequest { card_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}

pub async fn get_by_query(uid: u32, mut services: Services, f: QueryForm) -> ApiResult {
  let res: AccountForm = services
    .loyalty
    .get_account_by_query(QueryRequest {
      customer_id: f.customer_id,
      birthdate: f.birthdate,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}

pub async fn get_transactions(account_id: String, uid: u32, mut services: Services) -> ApiResult {
  let mut all = services
    .loyalty
    .get_transactions_all(TransactionAllRequest { account_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<TransactionForm> = Vec::new();
  while let Some(tr) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(tr.into());
  }

  Ok(warp::reply::json(&result))
}

pub async fn set_card(uid: u32, mut services: Services, f: SetCardForm) -> ApiResult {
  let res: AccountForm = services
    .loyalty
    .set_card(Card {
      set_to_account_id: f.account_id,
      card_id: f.card_id,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}

pub async fn set_loyalty_level(
  uid: u32,
  mut services: Services,
  f: SetLoyaltyLevelForm,
) -> ApiResult {
  let res: AccountForm = services
    .loyalty
    .set_loyalty_level(LoyaltyLevelRequest {
      account_id: f.account_id,
      loyalty_level: f.loyalty_level,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}

pub async fn set_birthdate(uid: u32, mut services: Services, f: SetBirthdateForm) -> ApiResult {
  let res: AccountForm = services
    .loyalty
    .set_birthdate(SetBirthdateRequest {
      account_id: f.account_id,
      birthdate: f.birthdate,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}
