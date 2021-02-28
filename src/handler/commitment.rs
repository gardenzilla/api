use std::convert::{TryFrom, TryInto};

use crate::{prelude::*, services::Services};
use gzlib::proto::commitment::{
  AddCommitmentRequest, CommitmentObj, CustomerObj, CustomerRequest, PurchaseInfo,
};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerForm {
  customer_id: u32,
  commitments: Vec<CommitmentForm>,
}

impl From<CustomerObj> for CustomerForm {
  fn from(f: CustomerObj) -> Self {
    Self {
      customer_id: f.customer_id,
      commitments: f.commitments.into_iter().map(|i| i.into()).collect(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitmentForm {
  commitment_id: String,
  customer_id: u32,
  target: u32,
  discount_percentage: u32,
  valid_still: String,
  balance: u32,
  purchase_log: Vec<PurchaseInfoForm>,
  is_withdrawn: bool,
  is_active: bool,
  created_by: u32,
  created_at: String,
}

impl From<CommitmentObj> for CommitmentForm {
  fn from(f: CommitmentObj) -> Self {
    Self {
      commitment_id: f.commitment_id,
      customer_id: f.customer_id,
      target: f.target,
      discount_percentage: f.discount_percentage,
      valid_still: f.valid_till,
      balance: f.balance,
      purchase_log: f.purchase_log.into_iter().map(|i| i.into()).collect(),
      is_withdrawn: f.is_withdrawn,
      is_active: f.is_active,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseInfoForm {
  purchase_id: String,
  total_net: u32,
  total_gross: u32,
  applied_discount: u32,
  removed: bool,
  created_at: String,
}

impl From<PurchaseInfo> for PurchaseInfoForm {
  fn from(f: PurchaseInfo) -> Self {
    Self {
      purchase_id: f.purchase_id,
      total_net: f.total_net,
      total_gross: f.total_gross,
      applied_discount: f.applied_discount,
      removed: f.removed,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitmentInfoForm {
  active_commitment: Option<CommitmentInfo>,
  has_active_commitment: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitmentInfo {
  commitment_id: String,
  customer_id: u32,
  target: u32,
  discount_percentage: u32,
  balance: u32,
  is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCommitmentForm {
  customer_id: u32,
  target: u32,
  discount_percentage: u32,
}

pub async fn get_all(uid: u32, mut services: Services) -> ApiResult {
  let res = services
    .commitment
    .get_customer_ids(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .customer_ids;
  Ok(reply::json(&res))
}

pub async fn add_commitment(uid: u32, mut services: Services, f: AddCommitmentForm) -> ApiResult {
  let res: CustomerForm = services
    .commitment
    .add_commitment(AddCommitmentRequest {
      customer_id: f.customer_id,
      target: f.target,
      discount_percentage: f.discount_percentage,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn get_customer(customer_id: u32, uid: u32, mut services: Services) -> ApiResult {
  let res: CustomerForm = services
    .commitment
    .get_customer(CustomerRequest { customer_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn has_active_commitment(uid: u32, mut services: Services) -> ApiResult {
  todo!()
}

pub async fn has_active_commitment_bulk(uid: u32, mut services: Services) -> ApiResult {
  todo!()
}
