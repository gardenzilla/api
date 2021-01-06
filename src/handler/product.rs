use crate::{prelude::*, services::Services};
use gzlib::proto::product::{
  FindProductRequest, FindSkuRequest, GetProductBulkRequest, GetProductRequest, GetSkuBulkRequest,
  GetSkuRequest, NewProduct, NewSku, ProductObj, SkuObj, UpdateSkuDivideRequest,
};
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProductForm {
  name: String,
  description: String,
  unit: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSkuForm {
  product_id: u32,
  sub_name: String,
  quantity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductForm {
  pub product_id: u32,
  pub name: String,
  pub description: String,
  pub unit: String,
  pub skus: Vec<u32>,
  pub created_at: String,
  pub created_by: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkuForm {
  pub sku: u32,
  pub product_id: u32,
  pub subname: String,
  pub display_name: String,
  pub display_packaging: String,
  pub quantity: String,
  pub unit: String,
  pub can_divide: bool,
  pub created_at: String,
  pub created_by: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindForm {
  pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkuSetDivideForm {
  pub sku: u32,
  pub can_divide: bool,
}

impl From<NewProduct> for NewProductForm {
  fn from(np: NewProduct) -> Self {
    Self {
      name: np.name,
      description: np.description,
      unit: np.unit,
    }
  }
}

impl From<ProductObj> for ProductForm {
  fn from(p: ProductObj) -> Self {
    Self {
      product_id: p.product_id,
      name: p.name,
      description: p.description,
      unit: p.unit,
      skus: p.skus,
      created_at: p.created_at,
      created_by: p.created_by,
    }
  }
}

impl From<SkuObj> for SkuForm {
  fn from(s: SkuObj) -> Self {
    Self {
      sku: s.sku,
      product_id: s.product_id,
      subname: s.subname,
      display_name: s.display_name,
      display_packaging: s.display_packaging,
      quantity: s.quantity,
      unit: s.unit,
      can_divide: s.can_divide,
      created_at: s.created_at,
      created_by: s.created_by,
    }
  }
}

pub async fn create_product(uid: u32, mut services: Services, np: NewProductForm) -> ApiResult {
  let product: ProductForm = services
    .product
    .create_product(NewProduct {
      name: np.name,
      description: np.description,
      unit: np.unit,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&product))
}

pub async fn get_product_all(_: u32, mut services: Services) -> ApiResult {
  let all = services
    .product
    .get_product_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  Ok(warp::reply::json(&all.product_ids))
}

pub async fn get_product_by_id(pid: u32, _: u32, mut services: Services) -> ApiResult {
  let product: ProductForm = services
    .product
    .get_product(GetProductRequest { product_id: pid })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(warp::reply::json(&product))
}

pub async fn get_product_bulk(_: u32, mut services: Services, product_ids: Vec<u32>) -> ApiResult {
  let mut products = services
    .product
    .get_product_bulk(GetProductBulkRequest { product_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<ProductForm> = Vec::new();
  while let Some(user) = products.message().await.map_err(|e| ApiError::from(e))? {
    result.push(user.into());
  }
  Ok(warp::reply::json(&result))
}

pub async fn update_product(
  pid: u32,
  uid: u32,
  mut services: Services,
  p: ProductForm,
) -> ApiResult {
  let product: ProductForm = services
    .product
    .update_product(ProductObj {
      product_id: pid,
      name: p.name,
      description: p.description,
      unit: p.unit,
      skus: p.skus,
      created_by: p.created_by,
      created_at: p.created_at,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&product))
}

pub async fn find_product(uid: u32, mut services: Services, f: FindForm) -> ApiResult {
  let product = services
    .product
    .find_product(FindProductRequest { query: f.query })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .sku_ids;
  Ok(reply::json(&product))
}

pub async fn create_sku(uid: u32, mut services: Services, ns: NewSkuForm) -> ApiResult {
  let sku: SkuForm = services
    .product
    .create_sku(NewSku {
      product_id: ns.product_id,
      sub_name: ns.sub_name,
      quantity: ns.quantity,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&sku))
}

pub async fn get_sku_all(_: u32, mut services: Services) -> ApiResult {
  let all = services
    .product
    .get_sku_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();
  Ok(warp::reply::json(&all.sku_ids))
}

pub async fn get_sku_by_id(sid: u32, _: u32, mut services: Services) -> ApiResult {
  let sku: SkuForm = services
    .product
    .get_sku(GetSkuRequest { sku_id: sid })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(warp::reply::json(&sku))
}

pub async fn get_sku_bulk(_: u32, mut services: Services, sku_ids: Vec<u32>) -> ApiResult {
  let mut skus = services
    .product
    .get_sku_bulk(GetSkuBulkRequest { sku_id: sku_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<SkuForm> = Vec::new();
  while let Some(sku) = skus.message().await.map_err(|e| ApiError::from(e))? {
    result.push(sku.into());
  }
  Ok(warp::reply::json(&result))
}

pub async fn update_sku(sku: u32, uid: u32, mut services: Services, s: SkuForm) -> ApiResult {
  let sku: SkuForm = services
    .product
    .update_sku(SkuObj {
      sku,
      product_id: s.product_id,
      subname: s.subname,
      display_name: s.display_name,
      display_packaging: s.display_packaging,
      quantity: s.quantity,
      unit: s.unit,
      can_divide: s.can_divide,
      created_by: s.created_by,
      created_at: s.created_at,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&sku))
}

pub async fn find_sku(uid: u32, mut services: Services, f: FindForm) -> ApiResult {
  let sku = services
    .product
    .find_sku(FindSkuRequest { query: f.query })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .sku_ids;
  Ok(reply::json(&sku))
}

pub async fn sku_set_divide(uid: u32, mut services: Services, f: SkuSetDivideForm) -> ApiResult {
  let sku: SkuForm = services
    .product
    .update_sku_divide(UpdateSkuDivideRequest {
      sku: f.sku,
      can_divide: f.can_divide,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&sku))
}
