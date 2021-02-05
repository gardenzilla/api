use crate::{prelude::*, services::Services};
use futures_util::stream;
use gzlib::proto::{
  email::EmailRequest,
  pricing::{GetPriceBulkRequest, PriceObject},
  procurement::{
    AddSkuRequest, AddUplRequest, CreateNewRequest, GetByIdRequest, GetInfoBulkRequest,
    ProcurementInfoObject, ProcurementItem, ProcurementObject, RemoveRequest, RemoveSkuRequest,
    RemoveUplRequest, SetDeliveryDateRequest, SetReferenceRequest, SetSkuPieceRequest,
    SetSkuPriceRequest, SetStatusRequest, Status, UpdateUplRequest, UplCandidate,
  },
  product::{GetSkuBulkRequest, SkuObj},
  upl::{UplNew, UplObj},
};
use serde::{Deserialize, Serialize};
use tonic::Request;
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
  opened_sku: bool,
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
      opened_sku: u.opened_sku,
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
        opened_sku: f.upl_candidate.opened_sku,
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

// 1. Try to get ProcurementObject
// 2. Check status
// 3. Try to create UPLs
// 4. Set status to closed
pub async fn set_status_closed(procurement_id: u32, uid: u32, mut services: Services) -> ApiResult {
  // 1. Try to get ProcurementObject
  let procurement_object: ProcurementForm = services
    .procurement
    .get_by_id(GetByIdRequest { procurement_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  // 2. Check if status is Processing
  match procurement_object.status {
    StatusForm::Processing => (),
    // If not, return error
    _ => {
      return Err(
        ApiError::bad_request("A beszerzés nem zárható le! A státusz nem 'Feldolgozás alatt'")
          .into(),
      )
    }
  }

  // Check if all new UPL IDS are not already taken
  let new_upl_ids = procurement_object
    .upls
    .iter()
    .map(|u| u.upl_id.clone())
    .collect::<Vec<String>>();

  let mut all_upls: Vec<UplObj> = Vec::new();

  let mut all_upl_stream = services
    .upl
    .get_bulk(gzlib::proto::upl::BulkRequest {
      upl_ids: new_upl_ids,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  while let Some(upl_obj) = all_upl_stream
    .message()
    .await
    .map_err(|e| ApiError::from(e))?
  {
    all_upls.push(upl_obj);
  }

  // If there is any found UPL with a new ID, then return error!
  if all_upls.len() > 0 {
    return Err(
      ApiError::bad_request(&format!(
        "A beszerzés nem zárható le. Az alábbi UPL azonosítók már használatban vannak: {:?}",
        all_upls.into_iter().map(|u| u.id).collect::<Vec<String>>(),
      ))
      .into(),
    );
  }

  // Collect SKU IDs
  let sku_id = procurement_object
    .items
    .iter()
    .map(|i| i.sku)
    .collect::<Vec<u32>>();

  // Load SKU objects to access SKU and product data
  let mut all_skus = services
    .product
    .get_sku_bulk(GetSkuBulkRequest {
      sku_id: sku_id.clone(),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut sku_objects: Vec<SkuObj> = Vec::new();

  while let Some(sku_obj) = all_skus.message().await.map_err(|e| ApiError::from(e))? {
    sku_objects.push(sku_obj);
  }

  // Load PriceObjects to access SKU price data
  let mut all_prices = services
    .pricing
    .get_price_bulk(GetPriceBulkRequest { skus: sku_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut price_objects: Vec<PriceObject> = Vec::new();

  while let Some(price_obj) = all_prices.message().await.map_err(|e| ApiError::from(e))? {
    price_objects.push(price_obj);
  }

  // Create empty result vector
  let mut result_upl_candidates: Vec<UplNew> = Vec::new();

  for sku_item in procurement_object.items.iter() {
    // Try find related SKU object
    let sku_obj =
      sku_objects
        .iter()
        .find(|so| so.sku == sku_item.sku)
        .ok_or(ApiError::bad_request(
          "A beszerzés nem létező SKUt tartalmaz!",
        ))?;

    // Try find related Price object
    let price_obj = price_objects
      .iter()
      .find(|po| po.sku == sku_item.sku)
      .ok_or(ApiError::bad_request(&format!(
        "A beszerzés alábbi SKUja nem rendelkezik eladási árral: #{}, {}",
        sku_item.sku, sku_obj.display_name
      )))?;

    // Collect UPLs related to this SKU item
    let mut u_candidates = procurement_object
      .upls
      .iter()
      .filter(|upl_candidate| upl_candidate.sku == sku_item.sku)
      .map(|uc| UplNew {
        upl_id: uc.upl_id.clone(),
        product_id: sku_obj.product_id,
        sku: uc.sku,
        best_before: uc.best_before.clone(),
        stock_id: 1, // todo: refact this value to grab from ENV variable
        procurement_id: procurement_object.id,
        is_opened: uc.opened_sku,
        created_by: uid,
        product_unit: sku_obj.unit.clone(),
        piece: uc.upl_piece,
        sku_divisible_amount: sku_obj.divisible_amount,
        sku_divisible: sku_obj.can_divide,
        sku_net_price: price_obj.price_net_retail,
        sku_vat: price_obj.vat.clone(),
        sku_gross_price: price_obj.price_gross_retail,
        procurement_net_price_sku: sku_item.expected_net_price,
      })
      .collect::<Vec<UplNew>>();

    // Check best_before if SKU is perishable
    if sku_obj.perishable {
      match u_candidates.iter().all(|uc| uc.best_before.len() > 0) {
        true => (),
        false => {
          return Err(
            ApiError::bad_request(&format!(
              "Az alábbi SKU romlandó, viszont nem minden UPL-hez van lejárat rögzítve: {}",
              &sku_obj.display_name
            ))
            .into(),
          )
        }
      }
    }

    // Check if all UPL count is the required one
    if u_candidates.iter().fold(0, |acc, uc| {
      acc
        + match uc.is_opened {
          true => 1,
          false => uc.piece,
        }
    }) != sku_item.ordered_amount
    {
      return Err(
        ApiError::bad_request(&format!(
          "A beszerzés nem zárható le! Az alábbi SKU nem rendelkezik minden UPL-el: {}",
          &sku_obj.display_name
        ))
        .into(),
      );
    }

    // Add SKU related upl candidates into the result upl candidates
    result_upl_candidates.append(&mut u_candidates);
  }

  // All UPL are fine, create request stream
  let request = Request::new(stream::iter(result_upl_candidates));

  // 4. Create UPLs
  let created_upl_ids = services
    .upl
    .create_new_bulk(request)
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .upl_ids;

  // Send email to sysadmin if not all UPLs are created!
  if procurement_object.upls.len() != created_upl_ids.len() {
    services
      .email
      .send_email(EmailRequest {
        to: "peter.mezei@gardenova.hu".to_string(),
        subject: "Proc hiba! Nem minden UPL jött létre!".to_string(),
        body: format!(
          "UPL létrehozás hiba! Nem minden UPL jött létre! Proc id: {}! {} helyett {}!",
          procurement_object.id,
          procurement_object.upls.len(),
          created_upl_ids.len()
        ),
      })
      .await
      .map_err(|e| ApiError::from(e))?;
  }

  // Set procurement status to closed
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

  // Reply updated procurement object
  Ok(reply::json(&res))
}
