use std::convert::{TryFrom, TryInto};

use crate::{prelude::*, services::Services};
use gzlib::proto::upl::{
  upl_obj::{Depreciation as SDepreciation, Kind, Location as SLocation, Lock as SLock},
  BulkRequest, ByIdRequest, UplObj,
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
pub enum UplKind {
  Sku {
    sku: u32,
  },
  BulkSku {
    sku: u32,
    upl_pieces: u32,
  },
  OpenedSku {
    sku: u32,
    amount: u32,
    successors: Vec<String>,
  },
  DerivedProduct {
    derived_from: String,
    amount: u32,
  },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Depreciation {
  depreciation_id: u32,
  depreciation_comment: String,
  depreciation_net_price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Lock {
  CartLock(String),
  DeliveryLock(u32),
  InventoryLock(u32),
  None,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Location {
  Stock(u32),
  Delivery(u32),
  Cart(String),
  Discard(u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UplForm {
  upl_id: String,
  product_id: u32,
  upl_kind: UplKind,
  upl_piece: u32,
  is_healthy: bool,
  best_before: String,
  depreciation: Option<Depreciation>,
  procurement_id: u32,
  procurement_net_price: u32,
  is_divisible: bool,
  divisible_amount: u32,
  lock: Lock,
  location: Location,
  is_archived: bool,
  created_by: u32,
  created_at: String,
}

impl From<Kind> for UplKind {
  fn from(k: Kind) -> Self {
    match k {
      Kind::Sku(sku) => UplKind::Sku { sku: sku.sku },
      Kind::BulkSku(bulk_sku) => UplKind::BulkSku {
        sku: bulk_sku.sku,
        upl_pieces: bulk_sku.upl_pieces,
      },
      Kind::OpenedSku(opened_sku) => UplKind::OpenedSku {
        sku: opened_sku.sku,
        amount: opened_sku.amount,
        successors: opened_sku.successors,
      },
      Kind::DerivedProduct(derived_sku) => UplKind::DerivedProduct {
        derived_from: derived_sku.derived_from,
        amount: derived_sku.amount,
      },
    }
  }
}

impl From<SDepreciation> for Depreciation {
  fn from(sd: SDepreciation) -> Self {
    Self {
      depreciation_id: sd.depreciation_id,
      depreciation_comment: sd.depreciation_comment,
      depreciation_net_price: sd.depreciation_net_price,
    }
  }
}

impl From<SLock> for Lock {
  fn from(sl: SLock) -> Self {
    match sl {
      SLock::CartLock(id) => Self::CartLock(id),
      SLock::DeliveryLock(id) => Self::DeliveryLock(id),
      SLock::InventoryLock(id) => Self::InventoryLock(id),
      SLock::None(_) => Self::None,
    }
  }
}

impl From<SLocation> for Location {
  fn from(sl: SLocation) -> Self {
    match sl {
      SLocation::Stock(id) => Self::Stock(id),
      SLocation::Delivery(id) => Self::Delivery(id),
      SLocation::Cart(id) => Self::Cart(id),
      SLocation::Discard(id) => Self::Discard(id),
    }
  }
}

impl TryFrom<UplObj> for UplForm {
  type Error = ApiError;

  fn try_from(upl: UplObj) -> Result<Self, Self::Error> {
    let res = Self {
      upl_id: upl.id,
      product_id: upl.product_id,
      upl_kind: upl
        .kind
        .ok_or(ApiError::internal_error("Missing UPL KIND"))?
        .into(),
      upl_piece: upl.upl_piece,
      is_healthy: upl.is_healty,
      best_before: upl.best_before,
      depreciation: match upl.depreciation {
        Some(dp) => Some(dp.into()),
        None => None,
      },
      procurement_id: upl.procurement_id,
      procurement_net_price: upl.procurement_net_price,
      is_divisible: upl.is_divisible,
      divisible_amount: upl.divisible_amount,
      lock: upl
        .lock
        .ok_or(ApiError::internal_error("Missing LOCK"))?
        .into(),
      location: upl
        .location
        .ok_or(ApiError::internal_error("Missing LOCATION"))?
        .into(),
      is_archived: upl.is_archived,
      created_by: upl.created_by,
      created_at: upl.created_at,
    };
    Ok(res)
  }
}

pub async fn get_upl_by_id(upl_id: String, _uid: u32, mut services: Services) -> ApiResult {
  let upl: UplForm = services
    .upl
    .get_by_id(ByIdRequest { upl_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&upl))
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
