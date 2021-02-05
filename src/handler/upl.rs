use std::{
  collections::HashMap,
  convert::{TryFrom, TryInto},
};

use crate::{prelude::*, services::Services};
use gzlib::proto::upl::{
  upl_obj::{Depreciation as SDepreciation, Kind, Location as SLocation, Lock as SLock},
  BulkRequest, ByIdRequest, BySkuAndLocationRequest, CloseUplRequest, DepreciationPriceRequest,
  DepreciationRequest, DivideRequest, LocationInfoBulkRequest, LocationInfoRequest,
  LocationInfoResponse, MergeRequest, OpenUplRequest, SplitRequest, UplObj,
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
pub struct UpdateBestBeforeForm {
  upl_id: String,
  best_before: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SplitForm {
  upl_id: String,
  new_upl: String,
  piece: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DivideForm {
  upl_id: String,
  new_upl: String,
  requested_amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetDepreciationForm {
  upl_id: String,
  depreciation_id: u32,
  depreciation_comment: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveDepreciationForm {
  upl_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetDepreciationPriceForm {
  upl_id: String,
  depreciation_net_price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveDepreciationPriceForm {
  upl_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenForm {
  upl_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseForm {
  upl_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MergeBackForm {
  upl_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetBySkuAndStockForm {
  sku: u32,
  stock_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLocationInfoForm {
  sku: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StockInfoForm {
  total: u32,
  healthy: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationInfoForm {
  sku: u32,
  stocks: HashMap<u32, StockInfoForm>,
}

impl From<LocationInfoResponse> for LocationInfoForm {
  fn from(f: LocationInfoResponse) -> Self {
    Self {
      sku: f.sku,
      stocks: f
        .stocks
        .into_iter()
        .map(|(k, v)| {
          (
            k,
            StockInfoForm {
              total: v.total,
              healthy: v.healthy,
            },
          )
        })
        .collect::<HashMap<u32, StockInfoForm>>(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UplKind {
  Sku,
  BulkSku,
  OpenedSku,
  DerivedProduct,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Lock {
  CartLock,
  DeliveryLock,
  InventoryLock,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Location {
  Stock,
  Delivery,
  Cart,
  Discard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UplForm {
  upl_id: String,
  product_id: u32,
  upl_kind: UplKind,
  sku: u32,
  pieces: u32,
  divisible_amount: u32,
  sku_divisible_amount: u32,
  derived_successors: Option<Vec<String>>,
  derived_from: Option<String>,
  is_healthy: bool,
  best_before: String,
  is_depreciated: bool,
  depreciation_id: Option<u32>,
  depreciation_comment: Option<String>,
  has_special_price: bool,
  procurement_id: u32,
  procurement_net_price: u32,
  procurement_net_price_sku: u32,
  is_divisible: bool,
  lock: Option<Lock>,
  lock_id: Option<String>,
  location: Location,
  location_id: String,
  product_unit: String,
  vat: String,
  price_net: u32,
  price_gross: u32,
  margin_net: i32,
  is_archived: bool,
  created_by: u32,
  created_at: String,
}

impl TryFrom<UplObj> for UplForm {
  type Error = ApiError;

  fn try_from(u: UplObj) -> Result<Self, Self::Error> {
    let res = Self {
      upl_id: u.id,
      product_id: u.product_id,
      upl_kind: match u
        .kind
        .as_ref()
        .ok_or(ApiError::internal_error("NO UPL KIND!"))?
      {
        Kind::Sku(_) => UplKind::Sku,
        Kind::BulkSku(_) => UplKind::BulkSku,
        Kind::OpenedSku(_) => UplKind::OpenedSku,
        Kind::DerivedProduct(_) => UplKind::DerivedProduct,
      },
      sku: u.sku_id,
      pieces: u.upl_piece,
      divisible_amount: match u
        .kind
        .as_ref()
        .ok_or(ApiError::internal_error("NO UPL KIND!"))?
      {
        Kind::Sku(_) => 1,
        Kind::BulkSku(_) => 1,
        Kind::OpenedSku(_opened_sku) => _opened_sku.amount,
        Kind::DerivedProduct(_derived_product) => _derived_product.amount,
      },
      sku_divisible_amount: u.sku_divisible_amount,
      derived_successors: match u
        .kind
        .as_ref()
        .ok_or(ApiError::internal_error("NO UPL KIND!"))?
      {
        Kind::OpenedSku(_opened_sku) => Some(_opened_sku.successors.to_owned()),
        _ => None,
      },
      derived_from: match u
        .kind
        .as_ref()
        .ok_or(ApiError::internal_error("NO UPL KIND!"))?
      {
        Kind::DerivedProduct(_derived_product) => Some(_derived_product.derived_from.to_string()),
        _ => None,
      },
      is_healthy: u.is_healty,
      best_before: u.best_before,
      is_depreciated: u.depreciation.is_some(),
      depreciation_id: match u.depreciation.as_ref() {
        Some(dep) => Some(dep.depreciation_id),
        None => None,
      },
      depreciation_comment: match u.depreciation.as_ref() {
        Some(dep) => Some(dep.depreciation_comment.to_string()),
        None => None,
      },
      has_special_price: u.has_special_price,
      procurement_id: u.procurement_id,
      procurement_net_price: u.procurement_net_price,
      procurement_net_price_sku: u.procurement_net_price_sku,
      is_divisible: u.is_divisible,
      lock: match u.lock.as_ref().ok_or(ApiError::internal_error("NO LOCK"))? {
        SLock::CartLock(lid) => Some(Lock::CartLock),
        SLock::DeliveryLock(lid) => Some(Lock::DeliveryLock),
        SLock::InventoryLock(lid) => Some(Lock::InventoryLock),
        SLock::None(_) => None,
      },
      lock_id: match u.lock.as_ref().ok_or(ApiError::internal_error("NO LOCK"))? {
        SLock::CartLock(lid) => Some(lid.to_string()),
        SLock::DeliveryLock(lid) => Some(lid.to_string()),
        SLock::InventoryLock(lid) => Some(lid.to_string()),
        SLock::None(_) => None,
      },
      location: match u
        .location
        .as_ref()
        .ok_or(ApiError::internal_error("NO LOCATION"))?
      {
        SLocation::Stock(_) => Location::Stock,
        SLocation::Delivery(_) => Location::Delivery,
        SLocation::Cart(_) => Location::Cart,
        SLocation::Discard(_) => Location::Discard,
      },
      location_id: match u
        .location
        .as_ref()
        .ok_or(ApiError::internal_error("NO LOCATION"))?
      {
        SLocation::Stock(lid) => lid.to_string(),
        SLocation::Delivery(lid) => lid.to_string(),
        SLocation::Cart(lid) => lid.to_string(),
        SLocation::Discard(lid) => lid.to_string(),
      },
      product_unit: u.product_unit,
      vat: u.vat,
      price_net: u.price_net,
      price_gross: u.price_gross,
      margin_net: u.margin_net,
      is_archived: u.is_archived,
      created_by: u.created_by,
      created_at: u.created_at,
    };
    Ok(res)
  }
}

pub async fn get_upl_by_id(upl_id: String, _uid: u32, mut services: Services) -> ApiResult {
  let mut res: Option<UplForm> = None;
  match services.upl.get_by_id(ByIdRequest { upl_id }).await {
    Ok(upl) => {
      res = Some(upl.into_inner().try_into()?);
    }
    Err(_) => (),
  }
  Ok(reply::json(&res))
}

pub async fn get_upl_by_id_archive(upl_id: String, _uid: u32, mut services: Services) -> ApiResult {
  let mut res: Option<UplForm> = None;
  match services.upl.get_by_id_archive(ByIdRequest { upl_id }).await {
    Ok(upl) => {
      res = Some(upl.into_inner().try_into()?);
    }
    Err(_) => (),
  }
  Ok(reply::json(&res))
}

pub async fn get_upl_bulk(_uid: u32, mut services: Services, upl_ids: Vec<String>) -> ApiResult {
  let mut all = services
    .upl
    .get_bulk(BulkRequest { upl_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<UplForm> = Vec::new();
  while let Some(upl) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(upl.try_into()?);
  }
  Ok(warp::reply::json(&result))
}

pub async fn split_upl(uid: u32, mut services: Services, f: SplitForm) -> ApiResult {
  let upl: UplForm = services
    .upl
    .split(SplitRequest {
      upl: f.upl_id,
      new_upl: f.new_upl,
      piece: f.piece,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&upl))
}

pub async fn divide_upl(uid: u32, mut services: Services, f: DivideForm) -> ApiResult {
  let upl: UplForm = services
    .upl
    .divide(DivideRequest {
      upl: f.upl_id,
      new_upl: f.new_upl,
      requested_amount: f.requested_amount,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&upl))
}

pub async fn open(_uid: u32, mut services: Services, f: OpenForm) -> ApiResult {
  let upl: UplForm = services
    .upl
    .open_upl(OpenUplRequest { upl_id: f.upl_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&upl))
}

pub async fn close(_uid: u32, mut services: Services, f: CloseForm) -> ApiResult {
  let upl: UplForm = services
    .upl
    .close_upl(CloseUplRequest { upl_id: f.upl_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&upl))
}

pub async fn merge_back(uid: u32, mut services: Services, f: MergeBackForm) -> ApiResult {
  let _ = services
    .upl
    .merge_back(MergeRequest {
      upl_to_merge_back: f.upl_id,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?;

  Ok(reply::json(&()))
}

pub async fn get_by_sku_stock(
  _uid: u32,
  mut services: Services,
  f: GetBySkuAndStockForm,
) -> ApiResult {
  let upls: Vec<String> = services
    .upl
    .get_by_sku_and_location(BySkuAndLocationRequest {
      sku: f.sku,
      location: Some(gzlib::proto::upl::by_sku_and_location_request::Location::Stock(f.stock_id)),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .upl_ids;

  Ok(reply::json(&()))
}

pub async fn get_location_info(
  uid: u32,
  mut services: Services,
  f: GetLocationInfoForm,
) -> ApiResult {
  let res: LocationInfoForm = services
    .upl
    .get_location_info(LocationInfoRequest { sku: f.sku })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}

pub async fn get_location_info_bulk(_uid: u32, mut services: Services, f: Vec<u32>) -> ApiResult {
  let mut all = services
    .upl
    .get_location_info_bulk(LocationInfoBulkRequest { sku: f })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<LocationInfoForm> = Vec::new();
  while let Some(linfo) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(linfo.into());
  }
  Ok(warp::reply::json(&result))
}

pub async fn set_depreciation(
  uid: u32,
  mut services: Services,
  f: SetDepreciationForm,
) -> ApiResult {
  let res: UplForm = services
    .upl
    .set_depreciation(DepreciationRequest {
      upl: f.upl_id,
      depreciation_id: f.depreciation_id,
      depreciation_comment: f.depreciation_comment,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  Ok(reply::json(&res))
}

pub async fn set_depreciation_price(
  uid: u32,
  mut services: Services,
  f: SetDepreciationPriceForm,
) -> ApiResult {
  let res: UplForm = services
    .upl
    .set_depreciation_price(DepreciationPriceRequest {
      upl: f.upl_id,
      depreciation_net_price: f.depreciation_net_price,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  Ok(reply::json(&res))
}
