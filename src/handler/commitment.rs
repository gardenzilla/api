use std::convert::{TryFrom, TryInto};

use crate::{prelude::*, services::Services};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerForm {
  customer_id: u32,
  commitments: Vec<CommitmentForm>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitmentForm {
  commitment_id: String,
  customer_id: u32,
  target: u32,
  discount_percentage: u32,
  valid_still: String,
  balance: u32,
  purchase_log: Vec<PurchaseInfo>,
  is_withdrawn: bool,
  is_active: bool,
  created_by: u32,
  created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseInfo {
  purchase_id: String,
  total_net: u32,
  total_gross: u32,
  applied_discount: u32,
  removed: bool,
  created_at: String,
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

pub async fn get_customer_ids(uid: u32, mut services: Services) -> ApiResult {
  todo!()
}

pub async fn add_commitment(uid: u32, mut services: Services) -> ApiResult {
  todo!()
}

pub async fn get_customer(uid: u32, mut services: Services) -> ApiResult {
  todo!()
}

pub async fn has_active_commitment(uid: u32, mut services: Services) -> ApiResult {
  todo!()
}

pub async fn has_active_commitment_bulk(uid: u32, mut services: Services) -> ApiResult {
  todo!()
}
