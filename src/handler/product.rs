use crate::{prelude::*, services::Services};
use gzlib::proto::{
  product::{
    self, FindProductRequest, FindSkuRequest, GetProductBulkRequest, GetProductRequest,
    GetSkuBulkRequest, GetSkuRequest, NewProduct, NewSku, ProductObj, SkuObj,
    UpdateProductDiscontinuedRequest, UpdateSkuDiscontinuedRequest, UpdateSkuDivideRequest,
  },
  upl::{ByProductRequest, BySkuRequest, SetProductUnitRequest, SetSkuDivisibleRequest},
};
use product::UpdateProductPerishableRequest;
use serde::{Deserialize, Serialize};
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
  pub discontinued: bool,
  pub perishable: bool,
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
  pub divisible_amount: u32,
  pub unit: String,
  pub can_divide: bool,
  pub discontinued: bool,
  pub perishable: bool,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SkuSetDiscontinuedForm {
  pub sku: u32,
  pub discontinued: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSetPerishableForm {
  pub product_id: u32,
  pub perishable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSetDiscontinuedForm {
  pub product_id: u32,
  pub discontinued: bool,
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
      discontinued: p.discontinued,
      perishable: p.perishable,
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
      divisible_amount: s.divisible_amount,
      unit: s.unit,
      can_divide: s.can_divide,
      discontinued: s.discontinued,
      perishable: s.perishable,
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
  _uid: u32,
  mut services: Services,
  p: ProductForm,
) -> ApiResult {
  // Get current product version
  let current_product: ProductForm = services
    .product
    .get_product(GetProductRequest { product_id: pid })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  // Check if we update unit
  if current_product.unit != p.unit {
    // Check if we have managed UPLs
    // If we have any, dont update it!
    if services
      .upl
      .get_by_product(ByProductRequest { product_id: pid })
      .await
      .map_err(|e| ApiError::from(e))?
      .into_inner()
      .upl_ids
      .len()
      > 0
    {
      return Err(
        ApiError::bad_request(
          "A termék mértékegysége nem változtatható! Már van raktáron hozzá UPL!",
        )
        .into(),
      );
    }

    // Update UPLs product unit
    services
      .upl
      .set_product_unit(SetProductUnitRequest {
        product_id: pid,
        unit: p.unit.clone(),
      })
      .await
      .map_err(|e| ApiError::from(e))?;
  }

  // Update product
  let product: ProductForm = services
    .product
    .update_product(ProductObj {
      product_id: pid,
      name: p.name,
      description: p.description,
      unit: p.unit,
      skus: p.skus,
      discontinued: p.discontinued,
      perishable: p.perishable,
      created_by: p.created_by,
      created_at: p.created_at,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&product))
}

pub async fn find_product(_uid: u32, mut services: Services, f: FindForm) -> ApiResult {
  let product = services
    .product
    .find_product(FindProductRequest { query: f.query })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .sku_ids;
  Ok(reply::json(&product))
}

pub async fn product_set_discontinued(
  _uid: u32,
  mut services: Services,
  f: ProductSetDiscontinuedForm,
) -> ApiResult {
  let sku: ProductForm = services
    .product
    .update_product_discontinued(UpdateProductDiscontinuedRequest {
      product_id: f.product_id,
      discontinued: f.discontinued,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&sku))
}

pub async fn product_set_perishable(
  _uid: u32,
  mut services: Services,
  f: ProductSetPerishableForm,
) -> ApiResult {
  let sku: ProductForm = services
    .product
    .update_product_perishable(UpdateProductPerishableRequest {
      product_id: f.product_id,
      perishable: f.perishable,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&sku))
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

pub async fn update_sku(sku: u32, _uid: u32, mut services: Services, s: SkuForm) -> ApiResult {
  // Get current SKU
  let current_sku: SkuForm = services
    .product
    .get_sku(GetSkuRequest { sku_id: sku })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  // Check if we update quantity
  if current_sku.quantity != s.quantity {
    // Check if we have managed UPLs
    // If we have any, dont update it!
    if services
      .upl
      .get_by_sku(BySkuRequest { sku: sku })
      .await
      .map_err(|e| ApiError::from(e))?
      .into_inner()
      .upl_ids
      .len()
      > 0
    {
      return Err(
        ApiError::bad_request("A SKU mennyisége nem változtatható! Már van raktáron hozzá UPL!")
          .into(),
      );
    }
  }

  // Update SKU
  let sku: SkuForm = services
    .product
    .update_sku(SkuObj {
      sku,
      product_id: s.product_id,
      subname: s.subname,
      display_name: s.display_name,
      display_packaging: s.display_packaging,
      quantity: s.quantity,
      divisible_amount: s.divisible_amount,
      unit: s.unit,
      can_divide: s.can_divide,
      discontinued: s.discontinued,
      perishable: s.perishable,
      created_by: s.created_by,
      created_at: s.created_at,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&sku))
}

pub async fn find_sku(_uid: u32, mut services: Services, f: FindForm) -> ApiResult {
  let sku = services
    .product
    .find_sku(FindSkuRequest { query: f.query })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .sku_ids;
  Ok(reply::json(&sku))
}

pub async fn sku_set_divide(_uid: u32, mut services: Services, f: SkuSetDivideForm) -> ApiResult {
  // Update SKU
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

  // Update UPLs
  services
    .upl
    .set_sku_divisible(SetSkuDivisibleRequest {
      sku: f.sku,
      divisible: f.can_divide,
    })
    .await
    .map_err(|e| ApiError::from(e))?;

  Ok(reply::json(&sku))
}

pub async fn sku_set_discontinued(
  _uid: u32,
  mut services: Services,
  f: SkuSetDiscontinuedForm,
) -> ApiResult {
  let sku: SkuForm = services
    .product
    .update_sku_discontinued(UpdateSkuDiscontinuedRequest {
      sku: f.sku,
      discontinued: f.discontinued,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&sku))
}
