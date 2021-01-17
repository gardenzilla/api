use crate::{prelude::*, services::Services};
use gzlib::proto::procurement::{
  AddSkuRequest, AddUplRequest, CreateNewRequest, GetByIdRequest, GetInfoBulkRequest,
  ProcurementInfoObject, ProcurementItem, ProcurementObject, RemoveRequest, RemoveSkuRequest,
  RemoveUplRequest, SetDeliveryDateRequest, SetReferenceRequest, SetSkuPieceRequest,
  SetSkuPriceRequest, SetStatusRequest, Status, UpdateUplRequest, UplCandidate,
};
use serde::{Deserialize, Serialize};
use warp::reply;

// [GET   ] /procurement/<ID>
// [GET   ] /procurement/all
// [POST  ] /procurement/new
// [POST  ] /procurement/bulk
// [DELETE] /procurement/remove
// [PUT   ] /procurement/set_devilery_date
// [PUT   ] /procurement/set_reference
// [PUT   ] /procurement/add_sku
// [PUT   ] /procurement/remove_sku
// [PUT   ] /procurement/set_sku_piece
// [PUT   ] /procurement/set_sku_price
// [PUT   ] /procurement/add_upl
// [PUT   ] /procurement/update_upl
// [PUT   ] /procurement/remove_upl
// [PUT   ] /procurement/set_status_ordered/<ID>
// [PUT   ] /procurement/set_status_arrived/<ID>
// [PUT   ] /procurement/set_status_processing/<ID>
// [PUT   ] /procurement/set_status_closed/<ID>

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProcurementForm {
  source_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatusForm {
  New,
  Ordered,
  Arrived,
  Processing,
  Closed,
}

impl From<Status> for StatusForm {
  fn from(s: Status) -> Self {
    match s {
      Status::New => StatusForm::New,
      Status::Ordered => StatusForm::Ordered,
      Status::Arrived => StatusForm::Arrived,
      Status::Processing => StatusForm::Processing,
      Status::Closed => StatusForm::Closed,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcurementForm {
  pub id: u32,
  pub source_id: u32,
  pub reference: String,
  pub estimated_delivery_date: String,
  pub items: Vec<ProcurementItemForm>,
  pub upls: Vec<UplCandidateForm>,
  pub status: StatusForm,
  pub created_at: String,
  pub created_by: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcurementItemForm {
  pub sku: u32,
  pub ordered_amount: u32,
  pub expected_net_price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UplCandidateForm {
  upl_id: String,
  sku: u32,
  upl_piece: u32,
  best_before: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcurementInfoForm {
  id: u32,
  source_id: u32,
  sku_count: u32,
  sku_piece_count: u32,
  upl_count: u32,
  estimated_delivery_date: String,
  status: StatusForm,
  created_at: String,
  created_by: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveUplForm {
  procurement_id: u32,
  upl_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUplForm {
  procurement_id: u32,
  upl_id: String,
  sku: u32,
  piece: u32,
  best_before: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddUplForm {
  procurement_id: u32,
  upl_candidate: UplCandidateForm,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetSkuPriceForm {
  procurement_id: u32,
  sku: u32,
  expected_net_price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetSkuPieceForm {
  procurement_id: u32,
  sku: u32,
  piece: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveSkuForm {
  procurement_id: u32,
  sku: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddSkuForm {
  procurement_id: u32,
  sku: ProcurementItemForm,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetReferenceForm {
  procurement_id: u32,
  reference: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SetDeliveryDateForm {
  procurement_id: u32,
  delivery_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveForm {
  procurement_id: u32,
}

impl From<ProcurementItem> for ProcurementItemForm {
  fn from(p: ProcurementItem) -> Self {
    Self {
      sku: p.sku,
      ordered_amount: p.ordered_amount,
      expected_net_price: p.expected_net_price,
    }
  }
}

impl From<UplCandidate> for UplCandidateForm {
  fn from(u: UplCandidate) -> Self {
    Self {
      upl_id: u.upl_id,
      sku: u.sku,
      upl_piece: u.upl_piece,
      best_before: u.best_before,
    }
  }
}

impl From<ProcurementObject> for ProcurementForm {
  fn from(p: ProcurementObject) -> Self {
    Self {
      id: p.id,
      source_id: p.source_id,
      reference: p.reference,
      estimated_delivery_date: p.estimated_delivery_date,
      items: p.items.into_iter().map(|i| i.into()).collect(),
      upls: p.upls.into_iter().map(|u| u.into()).collect(),
      status: Status::from_i32(p.status)
        .expect("Error while getting Status from i32")
        .into(),
      created_at: p.created_at,
      created_by: p.created_by,
    }
  }
}

impl From<ProcurementInfoObject> for ProcurementInfoForm {
  fn from(pi: ProcurementInfoObject) -> Self {
    Self {
      id: pi.id,
      source_id: pi.source_id,
      sku_count: pi.sku_count,
      sku_piece_count: pi.sku_piece_count,
      upl_count: pi.upl_count,
      estimated_delivery_date: pi.estimated_delivery_date,
      status: Status::from_i32(pi.status)
        .expect("Error while getting Status from i32")
        .into(),
      created_at: pi.created_at,
      created_by: pi.created_by,
    }
  }
}

pub async fn create_new(uid: u32, mut services: Services, f: NewProcurementForm) -> ApiResult {
  let product: ProcurementForm = services
    .procurement
    .create_new(CreateNewRequest {
      source_id: f.source_id,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&product))
}

pub async fn get_all(_: u32, mut services: Services) -> ApiResult {
  let all = services
    .procurement
    .get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  Ok(warp::reply::json(&all.procurement_ids))
}

pub async fn get_by_id(procurement_id: u32, _: u32, mut services: Services) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .get_by_id(GetByIdRequest { procurement_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(warp::reply::json(&res))
}

pub async fn get_bulk(_: u32, mut services: Services, procurement_ids: Vec<u32>) -> ApiResult {
  let mut all = services
    .procurement
    .get_info_bulk(GetInfoBulkRequest { procurement_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<ProcurementInfoForm> = Vec::new();
  while let Some(procurement) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(procurement.into());
  }
  Ok(warp::reply::json(&result))
}

pub async fn remove(procurement_id: u32, _uid: u32, mut services: Services) -> ApiResult {
  let res: () = services
    .procurement
    .remove(RemoveRequest { procurement_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn set_delivery_date(
  _uid: u32,
  mut services: Services,
  f: SetDeliveryDateForm,
) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .set_delivery_date(SetDeliveryDateRequest {
      procurement_id: f.procurement_id,
      delivery_date: f.delivery_date,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn set_reference(_uid: u32, mut services: Services, f: SetReferenceForm) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .set_reference(SetReferenceRequest {
      procurement_id: f.procurement_id,
      reference: f.reference,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn add_sku(_uid: u32, mut services: Services, f: AddSkuForm) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .add_sku(AddSkuRequest {
      procurement_id: f.procurement_id,
      sku: Some(ProcurementItem {
        sku: f.sku.sku,
        ordered_amount: f.sku.ordered_amount,
        expected_net_price: f.sku.expected_net_price,
      }),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn remove_sku(_uid: u32, mut services: Services, f: RemoveSkuForm) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .remove_sku(RemoveSkuRequest {
      procurement_id: f.procurement_id,
      sku: f.sku,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn set_sku_piece(_uid: u32, mut services: Services, f: SetSkuPieceForm) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .set_sku_piece(SetSkuPieceRequest {
      procurement_id: f.procurement_id,
      sku: f.sku,
      piece: f.piece,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn set_sku_price(_uid: u32, mut services: Services, f: SetSkuPriceForm) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .set_sku_price(SetSkuPriceRequest {
      procurement_id: f.procurement_id,
      sku: f.sku,
      expected_net_price: f.expected_net_price,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn add_upl(_uid: u32, mut services: Services, f: AddUplForm) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .add_upl(AddUplRequest {
      procurement_id: f.procurement_id,
      upl_candidate: Some(UplCandidate {
        upl_id: f.upl_candidate.upl_id,
        sku: f.upl_candidate.sku,
        upl_piece: f.upl_candidate.upl_piece,
        best_before: f.upl_candidate.best_before,
      }),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn remove_upl(_uid: u32, mut services: Services, f: RemoveUplForm) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .remove_upl(RemoveUplRequest {
      procurement_id: f.procurement_id,
      upl_id: f.upl_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn update_upl(_uid: u32, mut services: Services, f: UpdateUplForm) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .update_upl(UpdateUplRequest {
      procurement_id: f.procurement_id,
      upl_id: f.upl_id,
      sku: f.sku,
      piece: f.piece,
      best_before: f.best_before,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn set_status_ordered(
  procurement_id: u32,
  uid: u32,
  mut services: Services,
) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .set_status(SetStatusRequest {
      procurement_id,
      status: Status::Ordered.into(),
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn set_status_arrived(
  procurement_id: u32,
  uid: u32,
  mut services: Services,
) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .set_status(SetStatusRequest {
      procurement_id,
      status: Status::Arrived.into(),
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn set_status_processing(
  procurement_id: u32,
  uid: u32,
  mut services: Services,
) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .set_status(SetStatusRequest {
      procurement_id,
      status: Status::Processing.into(),
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn set_status_closed(procurement_id: u32, uid: u32, mut services: Services) -> ApiResult {
  let res: ProcurementForm = services
    .procurement
    .set_status(SetStatusRequest {
      procurement_id,
      status: Status::Closed.into(),
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}
