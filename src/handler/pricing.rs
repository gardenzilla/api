use crate::{prelude::*, services::Services};
use gzlib::proto::pricing::{
  GetPriceBulkRequest, GetPriceRequest, PriceChangesRequest, PriceHistoryObject, PriceObject,
  SetPriceRequest,
};
use serde::{Deserialize, Serialize};
use warp::reply;

// GET upl/<ID>

// GET upl/by_sku/<SKU>
// GET upl/by_stock

// PUT upl/set_best_before
// PUT upl/split
// PUT upl/divide
// PUT upl/set_depreciation
// PUT upl/remove_depreciation
// PUT upl/set_depreciation_price
// PUT upl/remove_depreciation_price

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceChangesForm {
  from: String, // RFC3339
  till: String, // RFC3339
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceForm {
  sku: u32,
  price_net_retail: u32,
  vat: String,
  price_gross_retail: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceHistoryForm {
  price_net_retail: u32,
  vat: String,
  price_gross_retail: u32,
  created_at: String,
  created_by: u32,
}

impl From<PriceHistoryObject> for PriceHistoryForm {
  fn from(pho: PriceHistoryObject) -> Self {
    Self {
      price_net_retail: pho.price_net_retail,
      vat: pho.vat,
      price_gross_retail: pho.price_gross_retail,
      created_at: pho.created_at,
      created_by: pho.created_by,
    }
  }
}

impl From<PriceObject> for PriceForm {
  fn from(po: PriceObject) -> Self {
    Self {
      sku: po.sku,
      price_net_retail: po.price_net_retail,
      vat: po.vat,
      price_gross_retail: po.price_gross_retail,
    }
  }
}

pub async fn create_new(uid: u32, mut services: Services, pf: PriceForm) -> ApiResult {
  let price_form: PriceForm = services
    .pricing
    .set_price(SetPriceRequest {
      sku: pf.sku,
      price_net_retail: pf.price_net_retail,
      vat: pf.vat,
      price_gross_retail: pf.price_gross_retail,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&price_form))
}

pub async fn get_by_id(sku: u32, _uid: u32, mut services: Services) -> ApiResult {
  let price_form: Option<PriceForm> =
    match services.pricing.get_price(GetPriceRequest { sku }).await {
      Ok(r) => Some(r.into_inner().into()),
      Err(_) => None,
    };
  Ok(reply::json(&price_form))
}

pub async fn get_bulk(_uid: u32, mut services: Services, skus: Vec<u32>) -> ApiResult {
  let mut all = services
    .pricing
    .get_price_bulk(GetPriceBulkRequest { skus })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<PriceForm> = Vec::new();
  while let Some(price) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(price.into());
  }
  Ok(reply::json(&result))
}

pub async fn get_price_history(sku: u32, _uid: u32, mut services: Services) -> ApiResult {
  let mut result: Vec<PriceHistoryForm> = Vec::new();

  let res = match services
    .pricing
    .get_price_history(GetPriceRequest { sku })
    .await
  {
    Ok(resp) => Some(resp.into_inner()),
    Err(_) => None,
  };

  if let Some(mut all) = res {
    while let Some(ph) = all.message().await.map_err(|e| ApiError::from(e))? {
      result.push(ph.into());
    }
  }

  Ok(warp::reply::json(&result))
}

pub async fn get_latest_price_changes(
  _uid: u32,
  mut services: Services,
  pcf: PriceChangesForm,
) -> ApiResult {
  let ids: Vec<u32> = services
    .pricing
    .get_latest_price_changes(PriceChangesRequest {
      date_from: pcf.from,
      date_till: pcf.till,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .price_ids;
  Ok(reply::json(&ids))
}
