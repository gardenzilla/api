use std::{
  convert::{TryFrom, TryInto},
  todo,
};

use crate::{
  prelude::*,
  services::{self, Services},
};
use gzlib::proto::{
  self,
  cash::NewTransaction,
  product::SkuObj,
  purchase::{upl_info_object::UplKindOpenedSku, CartInfoObject, CartObject},
};
use proto::{
  customer::GetByIdRequest,
  pricing::PriceObject,
  product::ProductObj,
  purchase::{
    upl_info_object::UplKindSku, CartBulkRequest, CartByIdRequest, CartNewRequest,
    CartRemoveSkuRequest, CartSetSkuPieceRequest, UplInfoObject,
  },
  upl::UplObj,
};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerForm {
  id: u32,
  name: String,
  zip: String,
  location: String,
  street: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartInfoForm {
  id: String,
  customer_name: String,
  upl_count: u32,
  item_names: Vec<String>,
  owner: u32,
  created_by: u32,
  created_at: String,
}

impl From<CartInfoObject> for CartInfoForm {
  fn from(f: CartInfoObject) -> Self {
    Self {
      id: f.cart_id,
      customer_name: f.customer_name,
      upl_count: f.upl_count,
      item_names: f.item_names,
      owner: f.owner,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UplKindForm {
  Sku { sku: u32, piece: u32 },
  OpenedSku { product_id: u32, amount: u32 },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UplInfoForm {
  upl_id: String,
  kind: UplKindForm,
  name: String,
  retail_price_net: u32,
  vat: String,
  retail_price_gross: u32,
  procurement_net_price: u32,
  best_before: String,
  depreciated: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemKindForm {
  Sku,
  DerivedProduct,
  DepreciatedProduct,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemForm {
  sku: u32,
  name: String,
  piece: u32,
  retail_price_net: u32,
  vat: String,
  retail_price_gross: u32,
  total_retail_price_net: u32,
  total_retail_price_gross: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PaymentKindForm {
  Cash,
  Card,
  Transfer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaymentForm {
  id: String,
  amount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartForm {
  ancestor: String,
  id: String,
  customer: Option<CustomerForm>,
  discount_percentage: u32,
  shopping_list: Vec<ItemForm>,
  upls_sku: Vec<UplInfoForm>,
  upls_unique: Vec<UplInfoForm>,
  total_net: u32,
  total_vat: u32,
  total_gross: u32,
  need_invoice: bool,
  payment_kind: PaymentKindForm,
  payments: Vec<PaymentForm>,
  payment_balance: i32,
  profit_net: i32,
  owner_uid: u32,
  store_id: u32,
  date_completion: String,
  payment_duedate: String,
  created_by: u32,
  created_at: String,
}

impl From<UplInfoObject> for UplInfoForm {
  fn from(s: UplInfoObject) -> Self {
    let k = s.upl_kind.unwrap(); // TODO! Refact this line!
    Self {
      upl_id: s.upl_id,
      kind: match k {
        proto::purchase::upl_info_object::UplKind::Sku(s) => UplKindForm::Sku {
          sku: s.sku,
          piece: s.piece,
        },
        proto::purchase::upl_info_object::UplKind::OpenedSku(os) => UplKindForm::OpenedSku {
          product_id: os.product_id,
          amount: os.amount,
        },
      },
      name: s.name,
      retail_price_net: s.retail_net_price,
      vat: s.vat,
      retail_price_gross: s.retail_gross_price,
      procurement_net_price: s.procurement_net_price,
      best_before: s.best_before,
      depreciated: s.depreciated,
    }
  }
}

impl TryFrom<CartObject> for CartForm {
  type Error = ApiError;

  fn try_from(f: CartObject) -> Result<Self, Self::Error> {
    let p: proto::purchase::PaymentKind =
      proto::purchase::PaymentKind::from_i32(f.payment_kind).unwrap(); // TODO! FIX IT

    let res = Self {
      ancestor: f.ancestor,
      id: f.id,
      customer: match f.customer {
        Some(c) => Some(CustomerForm {
          id: c.customer_id,
          name: c.name.clone(),
          zip: c.zip.clone(),
          location: c.location.clone(),
          street: c.street.clone(),
        }),
        None => None,
      },
      discount_percentage: f.discount_percentage,
      shopping_list: f
        .shopping_list
        .iter()
        .map(|i| ItemForm {
          sku: i.sku,
          name: i.name.clone(),
          piece: i.piece,
          retail_price_net: i.retail_price_net,
          vat: i.vat.clone(),
          retail_price_gross: i.retail_price_gross,
          total_retail_price_net: i.total_retail_price_net,
          total_retail_price_gross: i.total_retail_price_gross,
        })
        .collect(),
      upls_sku: f.upls_sku.iter().map(|u| u.clone().into()).collect(),
      upls_unique: f.upls_unique.iter().map(|u| u.clone().into()).collect(),
      total_net: f.total_net,
      total_vat: f.total_vat,
      total_gross: f.total_gross,
      need_invoice: f.need_invoice,
      payment_kind: match p {
        proto::purchase::PaymentKind::Cash => PaymentKindForm::Cash,
        proto::purchase::PaymentKind::Card => PaymentKindForm::Card,
        proto::purchase::PaymentKind::Transfer => PaymentKindForm::Transfer,
      },
      payments: f
        .payments
        .iter()
        .map(|p| PaymentForm {
          id: p.payment_id.clone(),
          amount: p.amount,
        })
        .collect(),
      payment_balance: f.payment_balance,
      profit_net: f.profit_net,
      owner_uid: f.owner_uid,
      store_id: f.store_id,
      date_completion: f.date_completion,
      payment_duedate: f.payment_duedate,
      created_by: f.created_by,
      created_at: f.created_at,
    };
    Ok(res)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewCartForm {
  store_id: u32,
  created_by: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartSetSkuPieceForm {
  cart_id: String,
  sku: u32,
  piece: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartSetOwnerForm {
  cart_id: String,
  owner_uid: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartSetStoreForm {
  cart_id: String,
  store_id: u32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CartAddCustomerForm {
  cart_id: String,
  customer_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartRemoveCustomerForm {
  cart_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartSetPaymentForm {
  cart_id: String,
  payment_kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartCloseForm {
  cart_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartAddSkuForm {
  cart_id: String,
  sku_id: u32,
  piece: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartRemoveSkuForm {
  cart_id: String,
  sku_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartAddUplForm {
  cart_id: String,
  upl_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartRemoveUplForm {
  cart_id: String,
  upl_id: String,
}

enum UKind {
  Sku {
    sku: u32,
    piece: u32,
  },
  OpenedSku {
    sku: u32,
    pid: u32,
    amount: u32,
  },
  DerivedProduct {
    derived_from_upl: String,
    pid: u32,
    amount: u32,
  },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CartAddPaymentForm {
  cart_id: String,
  kind: String,
  amount: i32,
}

pub async fn new_cart(uid: u32, mut services: Services, f: NewCartForm) -> ApiResult {
  let res: CartForm = services
    .purchase
    .cart_new(CartNewRequest {
      store_id: f.store_id,
      owner_id: uid,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  Ok(reply::json(&res))
}

pub async fn cart_get_all(_uid: u32, mut services: Services) -> ApiResult {
  let res: Vec<String> = services
    .purchase
    .cart_get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .cart_ids;
  Ok(reply::json(&res))
}

pub async fn cart_get_by_id(cart_id: String, _uid: u32, mut services: Services) -> ApiResult {
  let res: CartForm = services
    .purchase
    .cart_get_by_id(CartByIdRequest { cart_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&res))
}

pub async fn get_bulk(_uid: u32, mut services: Services, cart_ids: Vec<String>) -> ApiResult {
  let mut all = services
    .purchase
    .cart_get_info_bulk(CartBulkRequest { cart_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<CartInfoForm> = Vec::new();
  while let Some(cart_info) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(cart_info.into());
  }
  Ok(reply::json(&result))
}

pub async fn cart_add_customer(
  _uid: u32,
  mut services: Services,
  f: CartAddCustomerForm,
) -> ApiResult {
  // First query customer
  let customer: proto::customer::CustomerObj = services
    .customer
    .get_by_id(GetByIdRequest {
      customer_id: f.customer_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let res: CartForm = services
    .purchase
    .cart_add_customer(proto::purchase::CartAddCustomerReuqest {
      cart_id: f.cart_id,
      customer_id: customer.id,
      customer_name: customer.name,
      customer_zip: customer.address_zip,
      customer_location: customer.address_location,
      customer_street: customer.address_street,
      tax_number: customer.tax_number,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&res))
}

pub async fn cart_remove_customer(
  _uid: u32,
  mut services: Services,
  f: CartRemoveCustomerForm,
) -> ApiResult {
  let res: CartForm = services
    .purchase
    .cart_remove_customer(proto::purchase::CartRemoveCustomerRequest { cart_id: f.cart_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&res))
}

pub async fn cart_add_sku(_uid: u32, mut services: Services, f: CartAddSkuForm) -> ApiResult {
  // First query sku
  let sku_obj: SkuObj = services
    .product
    .get_sku(proto::product::GetSkuRequest { sku_id: f.sku_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Then query its price
  let sku_price: PriceObject = services
    .pricing
    .get_price(proto::pricing::GetPriceRequest { sku: f.sku_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Then query to add SKU to cart
  let res: CartForm = services
    .purchase
    .cart_add_sku(proto::purchase::CartAddSkuRequest {
      cart_id: f.cart_id,
      sku_id: f.sku_id,
      piece: f.piece,
      name: sku_obj.display_name,
      vat: sku_price.vat,
      retail_price_net: sku_price.price_net_retail,
      retail_price_gross: sku_price.price_gross_retail,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&res))
}

pub async fn cart_remove_sku(_uid: u32, mut services: Services, f: CartRemoveSkuForm) -> ApiResult {
  // Then query to add SKU to cart
  let res: CartForm = services
    .purchase
    .cart_remove_sku(CartRemoveSkuRequest {
      cart_id: f.cart_id,
      sku_id: f.sku_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&res))
}

pub async fn cart_set_sku_piece(
  _uid: u32,
  mut services: Services,
  f: CartSetSkuPieceForm,
) -> ApiResult {
  // Then query to add SKU to cart
  let res: CartForm = services
    .purchase
    .cart_set_sku_piece(CartSetSkuPieceRequest {
      cart_id: f.cart_id,
      sku: f.sku,
      piece: f.piece,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;
  Ok(reply::json(&res))
}

pub async fn cart_add_upl(uid: u32, mut services: Services, f: CartAddUplForm) -> ApiResult {
  // First query UPL
  let upl_obj: UplObj = services
    .upl
    .get_by_id(proto::upl::ByIdRequest { upl_id: f.upl_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Then query Cart
  let cart_obj: CartObject = services
    .purchase
    .cart_get_by_id(proto::purchase::CartByIdRequest { cart_id: f.cart_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Now validate that the UPL is in the same location as Cart
  match upl_obj.location.unwrap() {
    proto::upl::upl_obj::Location::Stock(stock_id) => {
      if stock_id != cart_obj.store_id {
        return Err(
          ApiError::bad_request(&format!(
            "A kért UPL nem a kosár lokációján van, nem tehető a kosárba! UPL: {}, kosár: {}",
            stock_id, cart_obj.store_id
          ))
          .into(),
        );
      }
    }
    _ => {
      return Err(
        ApiError::bad_request(
          "A kért UPL nem értékesíthető! Vagy selejtezett, eladott, vagy szállítás alatt van",
        )
        .into(),
      )
    }
  }

  // Now check whether it already has the lock
  match upl_obj.lock.unwrap() {
    proto::upl::upl_obj::Lock::CartLock(cart_id) => {
      if cart_obj.id == cart_id {
        return Err(ApiError::bad_request("A kért UPL már a kosárban van!").into());
      }
    }
    _ => (),
  }

  let ukind: UKind = match upl_obj.kind.unwrap() {
    proto::upl::upl_obj::Kind::Sku(s) => UKind::Sku {
      sku: s.sku,
      piece: 1,
    },
    proto::upl::upl_obj::Kind::BulkSku(s) => UKind::Sku {
      sku: s.sku,
      piece: s.upl_pieces,
    },
    proto::upl::upl_obj::Kind::OpenedSku(s) => UKind::OpenedSku {
      sku: s.sku,
      pid: upl_obj.product_id,
      amount: s.amount,
    },
    proto::upl::upl_obj::Kind::DerivedProduct(s) => UKind::DerivedProduct {
      derived_from_upl: s.derived_from,
      pid: upl_obj.product_id,
      amount: s.amount,
    },
  };

  let cart: CartForm = match ukind {
    UKind::Sku { sku, piece } => {
      // Query SKU
      let sku_obj: SkuObj = services
        .product
        .get_sku(proto::product::GetSkuRequest { sku_id: sku })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      // Then query its price
      let price_obj: PriceObject = services
        .pricing
        .get_price(proto::pricing::GetPriceRequest { sku: sku })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      // Build up Upl Info Object
      let upl_info_object = UplInfoObject {
        upl_id: upl_obj.id.clone(),
        name: sku_obj.display_name,
        retail_net_price: price_obj.price_net_retail,
        vat: price_obj.vat,
        retail_gross_price: price_obj.price_gross_retail,
        procurement_net_price: upl_obj.procurement_net_price,
        best_before: upl_obj.best_before.clone(),
        depreciated: upl_obj.depreciation.is_some(),
        upl_kind: Some(proto::purchase::upl_info_object::UplKind::Sku(UplKindSku {
          sku: sku,
          piece: piece,
        })),
      };

      // Add UplInfoObject to Cart
      services
        .purchase
        .cart_add_upl(proto::purchase::CartAddUplRequest {
          cart_id: cart_obj.id.clone(),
          upl: Some(upl_info_object),
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner()
        .try_into()?
    }
    UKind::OpenedSku { sku, pid, amount } => {
      // Query SKU
      let sku_obj: SkuObj = services
        .product
        .get_sku(proto::product::GetSkuRequest { sku_id: sku })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      // Check this to avoid zero division at price calculation
      if sku_obj.divisible_amount == 0 {
        return Err(
          ApiError::bad_request("A kért UPL bontott, de a termék már nem kimérhető! Állítsa át!")
            .into(),
        );
      }

      // Query Product
      let product_obj: ProductObj = services
        .product
        .get_product(proto::product::GetProductRequest {
          product_id: sku_obj.product_id,
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      // Then query its price
      let price_obj: PriceObject = services
        .pricing
        .get_price(proto::pricing::GetPriceRequest { sku: sku })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      // Build up Upl Info Object
      let upl_info_object = UplInfoObject {
        upl_id: upl_obj.id.clone(),
        name: format!("{}, {} {}", product_obj.name, amount, product_obj.unit),
        retail_net_price: ((price_obj.price_net_retail as f32 / sku_obj.divisible_amount as f32)
          * amount as f32)
          .round() as u32,
        vat: price_obj.vat,
        retail_gross_price: ((price_obj.price_gross_retail as f32
          / sku_obj.divisible_amount as f32)
          * amount as f32)
          .round() as u32,
        procurement_net_price: ((upl_obj.procurement_net_price as f32
          / sku_obj.divisible_amount as f32)
          * amount as f32)
          .round() as u32,
        best_before: upl_obj.best_before.clone(),
        depreciated: upl_obj.depreciation.is_some(),
        upl_kind: Some(proto::purchase::upl_info_object::UplKind::OpenedSku(
          UplKindOpenedSku {
            product_id: pid,
            amount: amount,
          },
        )),
      };

      // Add UplInfoObject to Cart
      services
        .purchase
        .cart_add_upl(proto::purchase::CartAddUplRequest {
          cart_id: cart_obj.id.clone(),
          upl: Some(upl_info_object),
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner()
        .try_into()?
    }
    UKind::DerivedProduct {
      derived_from_upl,
      pid,
      amount,
    } => {
      // Query parent UPL
      let parent_upl_obj: UplObj = services
        .upl
        .get_by_id(proto::upl::ByIdRequest {
          upl_id: derived_from_upl,
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      let parent_sku = match parent_upl_obj.kind.unwrap() {
        // Parent of a devirev product must be opened sku
        proto::upl::upl_obj::Kind::OpenedSku(o) => o.sku,
        _ => {
          return Err(
            ApiError::internal_error(
              "A kért kimért UPL szülője nem bontott termék! Lehetetlen hiba!",
            )
            .into(),
          )
        }
      };

      // Query SKU
      let sku_obj: SkuObj = services
        .product
        .get_sku(proto::product::GetSkuRequest { sku_id: parent_sku })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      // Check this to avoid zero division at price calculation
      if sku_obj.divisible_amount == 0 {
        return Err(
          ApiError::bad_request("A kért UPL bontott, de a termék már nem kimérhető! Állítsa át!")
            .into(),
        );
      }

      // Query Product
      let product_obj: ProductObj = services
        .product
        .get_product(proto::product::GetProductRequest {
          product_id: sku_obj.product_id,
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      // Then query its price
      let price_obj: PriceObject = services
        .pricing
        .get_price(proto::pricing::GetPriceRequest { sku: parent_sku })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();

      // Build up Upl Info Object
      let upl_info_object = UplInfoObject {
        upl_id: upl_obj.id.clone(),
        name: format!("{}, {} {}", product_obj.name, amount, product_obj.unit),
        retail_net_price: ((price_obj.price_net_retail as f32 / sku_obj.divisible_amount as f32)
          * amount as f32)
          .round() as u32,
        vat: price_obj.vat,
        retail_gross_price: ((price_obj.price_gross_retail as f32
          / sku_obj.divisible_amount as f32)
          * amount as f32)
          .round() as u32,
        procurement_net_price: ((upl_obj.procurement_net_price as f32
          / sku_obj.divisible_amount as f32)
          * amount as f32)
          .round() as u32,
        best_before: upl_obj.best_before.clone(),
        depreciated: upl_obj.depreciation.is_some(),
        upl_kind: Some(proto::purchase::upl_info_object::UplKind::OpenedSku(
          UplKindOpenedSku {
            product_id: pid,
            amount: amount,
          },
        )),
      };

      // Add UplInfoObject to Cart
      services
        .purchase
        .cart_add_upl(proto::purchase::CartAddUplRequest {
          cart_id: cart_obj.id.clone(),
          upl: Some(upl_info_object),
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner()
        .try_into()?
    }
  };

  // Try to lock UPL to this cart
  match services
    .upl
    .lock_to_cart(proto::upl::CartLockRequest {
      upl: upl_obj.id.clone(),
      cart_id: cart_obj.id.clone(),
      created_by: uid,
    })
    .await
  {
    Ok(res) => (),
    Err(_) => {
      services
        .purchase
        .cart_remove_upl(proto::purchase::CartRemoveUplRequest {
          cart_id: cart_obj.id,
          upl_id: upl_obj.id,
        })
        .await
        .map_err(|e| ApiError::from(e))?;
    }
  }

  Ok(reply::json(&cart))
}

pub async fn cart_remove_upl(uid: u32, mut services: Services, f: CartRemoveUplForm) -> ApiResult {
  // Try to get UPL
  let upl_obj: UplObj = services
    .upl
    .get_by_id(proto::upl::ByIdRequest { upl_id: f.upl_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Check if UPL is locked into this cart
  // Return error if not
  match upl_obj.lock.unwrap() {
    proto::upl::upl_obj::Lock::CartLock(cart_id) => {
      if cart_id != f.cart_id {
        return Err(ApiError::bad_request("Az adott UPL egy másik kosárhoz van rendelve!").into());
      }
    }
    _ => return Err(ApiError::bad_request("Az adott UPL nincs a kosárban!").into()),
  }

  // Try to release lock
  services
    .upl
    .release_lock_from_cart(proto::upl::CartUnlockRequest {
      upl: upl_obj.id.clone(),
      cart_id: f.cart_id.clone(),
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Try to remove from cart
  let res: CartForm = services
    .purchase
    .cart_remove_upl(proto::purchase::CartRemoveUplRequest {
      cart_id: f.cart_id.clone(),
      upl_id: upl_obj.id.clone(),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  Ok(reply::json(&res))
}

pub async fn cart_set_payment(
  _uid: u32,
  mut services: Services,
  f: CartSetPaymentForm,
) -> ApiResult {
  let res: CartForm = services
    .purchase
    .cart_set_payment(proto::purchase::CartSetPaymentRequest {
      cart_id: f.cart_id,
      payment_kind: match f.payment_kind.as_str() {
        "Cash" | "cash" => proto::purchase::PaymentKind::Cash,
        "Card" | "card" => proto::purchase::PaymentKind::Card,
        "Transfer" | "transfer" => proto::purchase::PaymentKind::Transfer,
        _ => return Err(ApiError::bad_request("A megadott fizetési kód nem megfelelő!").into()),
      } as i32,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  Ok(reply::json(&res))
}

pub async fn cart_add_payment(
  uid: u32,
  mut services: Services,
  f: CartAddPaymentForm,
) -> ApiResult {
  // Check if cart valid
  let cart: CartForm = services
    .purchase
    .cart_get_by_id(proto::purchase::CartByIdRequest {
      cart_id: f.cart_id.clone(),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  // Do payment
  let transaction = services
    .cash
    .create_transaction(NewTransaction {
      kind: match f.kind.as_str() {
        "Cash" | "cash" => gzlib::proto::cash::TransactionKind::KindCash,
        "Card" | "card" => gzlib::proto::cash::TransactionKind::KindCard,
        "Transfer" | "transfer" => gzlib::proto::cash::TransactionKind::KindTransfer,
        _ => {
          return Err(ApiError::bad_request("A megadott tranzakció típus nem megfelelő!").into())
        }
      } as i32,
      amount: f.amount,
      reference: "".to_string(),
      comment: "".to_string(),
      created_by: uid,
      cart_id: Some(proto::cash::new_transaction::CartId::Cart(
        f.cart_id.clone(),
      )),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let res: CartForm = services
    .purchase
    .cart_add_payment(proto::purchase::CartAddPaymentRequest {
      cart_id: f.cart_id,
      payment_id: transaction.transaction_id,
      amount: transaction.amount,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  Ok(reply::json(&res))
}

pub async fn cart_close(uid: u32, mut services: Services, f: CartCloseForm) -> ApiResult {
  // Check if cart valid
  let cart: CartForm = services
    .purchase
    .cart_get_by_id(proto::purchase::CartByIdRequest {
      cart_id: f.cart_id.clone(),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  let cart_closed: CartForm = services
    .purchase
    .cart_close(proto::purchase::CartCloseRequest {
      cart_id: f.cart_id.clone(),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  // Move all locked UPLs into its cart
  services
    .upl
    .close_cart(proto::upl::CloseCartRequest {
      cart_id: f.cart_id,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?;

  // Return nothing if success
  Ok(reply::json(&""))
}
