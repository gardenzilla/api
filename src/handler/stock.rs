use crate::{prelude::*, services::Services};
use gzlib::proto::stock::{CreateNewRequest, GetByIdRequest, StockObject};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct StockForm {
  stock_id: u32,
  name: String,
  description: String,
  created_at: String,
  created_by: u32,
}

impl From<StockObject> for StockForm {
  fn from(so: StockObject) -> Self {
    Self {
      stock_id: so.stock_id,
      name: so.name,
      description: so.description,
      created_at: so.created_at,
      created_by: so.created_by,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewStockForm {
  name: String,
  description: String,
}

pub async fn create_new(uid: u32, mut services: Services, f: NewStockForm) -> ApiResult {
  let res: StockForm = services
    .stock
    .create_new(CreateNewRequest {
      name: f.name,
      description: f.description,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn get_by_id(stock_id: u32, _uid: u32, mut services: Services) -> ApiResult {
  let res: Option<StockForm> = match services.stock.get_by_id(GetByIdRequest { stock_id }).await {
    Ok(r) => Some(r.into_inner().into()),
    Err(_) => None,
  };
  Ok(reply::json(&res))
}

pub async fn update(stock_id: u32, _uid: u32, mut services: Services, f: StockForm) -> ApiResult {
  let res: StockForm = services
    .stock
    .update_by_id(StockObject {
      stock_id,
      name: f.name,
      description: f.description,
      created_by: f.created_by,
      created_at: f.created_at,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn get_all(_uid: u32, mut services: Services) -> ApiResult {
  let mut result: Vec<StockForm> = Vec::new();

  let mut all = services
    .stock
    .get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  while let Some(so) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(so.into());
  }

  Ok(warp::reply::json(&result))
}
