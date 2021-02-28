use std::{
  convert::{TryFrom, TryInto},
  todo,
};

use crate::{
  prelude::*,
  services::{self, Services},
};
use chrono::{DateTime, NaiveDate, Utc};
use gzlib::proto::{
  self,
  cash::NewTransaction,
  commitment::CustomerRequest,
  invoice::{
    invoice_form::{Customer, PaymentKind},
    InvoiceForm,
  },
  product::{GetProductRequest, GetSkuRequest, SkuObj},
  purchase::{
    upl_info_object::UplKindOpenedSku, AddCommitmentRequest, CartInfoObject, CartObject,
    CartSetDocumentRequest, DocumentKind, PurchaseByIdRequest, PurchaseSetInvoiceIdRequest,
    RemoveCommitmentRequest,
  },
  upl::upl_obj,
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

use super::purchase::PurchaseForm;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerForm {
  id: u32,
  name: String,
  zip: String,
  location: String,
  street: String,
  tax_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UplKindForm {
  Sku { sku: u32, piece: u32 },
  OpenedSku { product_id: u32, amount: u32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ItemKindForm {
  Sku,
  DerivedProduct,
  DepreciatedProduct,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentKindForm {
  Cash,
  Card,
  Transfer,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentForm {
  pub id: String,
  pub amount: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoyaltyTransaction {
  pub loyalty_account_id: String,
  pub transaction_id: String,
  pub burned_points: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartForm {
  ancestor: String,
  id: String,
  customer: Option<CustomerForm>,
  commitment_id: String,
  commitment_discount_percentage: u32,
  loyalty_card_id: String,
  loyalty_account_id: String,
  loyalty_level: String,
  shopping_list: Vec<ItemForm>,
  upls_sku: Vec<UplInfoForm>,
  upls_unique: Vec<UplInfoForm>,
  total_net: u32,
  total_vat: u32,
  total_gross: u32,
  commitment_discount_amount_gross: u32,
  burned_points: Vec<LoyaltyTransaction>,
  need_invoice: bool,
  payment_kind: PaymentKindForm,
  payments: Vec<PaymentForm>,
  payable: i32,
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
          tax_number: c.tax_number.clone(),
        }),
        None => None,
      },
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
      payable: f.payable,
      payment_balance: f.payment_balance,
      profit_net: f.profit_net,
      owner_uid: f.owner_uid,
      store_id: f.store_id,
      date_completion: f.date_completion,
      payment_duedate: f.payment_duedate,
      created_by: f.created_by,
      created_at: f.created_at,
      commitment_id: f.commitment_id,
      commitment_discount_percentage: f.commitment_discount_percentage,
      loyalty_card_id: f.loyalty_card_id,
      loyalty_account_id: f.loyalty_account_id,
      loyalty_level: f.loyalty_level,
      commitment_discount_amount_gross: f.commitment_discount_amount_gross,
      burned_points: f
        .burned_points
        .into_iter()
        .map(|tr| LoyaltyTransaction {
          loyalty_account_id: tr.loyalty_account_id,
          transaction_id: tr.transaction_id,
          burned_points: tr.burned_points,
        })
        .collect::<Vec<LoyaltyTransaction>>(),
    };
    Ok(res)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewCartForm {
  store_id: u32,
  created_by: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartSetSkuPieceForm {
  cart_id: String,
  sku: u32,
  piece: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartSetOwnerForm {
  cart_id: String,
  owner_uid: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartSetInvoiceForm {
  cart_id: String,
  need_invoice: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartSetStoreForm {
  cart_id: String,
  store_id: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartAddCustomerForm {
  cart_id: String,
  customer_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartRemoveCustomerForm {
  cart_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartSetPaymentForm {
  cart_id: String,
  payment_kind: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartCloseForm {
  cart_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartAddSkuForm {
  cart_id: String,
  sku_id: u32,
  piece: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartRemoveSkuForm {
  cart_id: String,
  sku_id: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartAddUplForm {
  cart_id: String,
  upl_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

  //
  // Add commitment if has any
  //
  // Query commitment (if has any)
  if let Ok(c) = services
    .commitment
    .has_active_commitment(CustomerRequest {
      customer_id: customer.id,
    })
    .await
  {
    let commitment = c.into_inner();
    // If has active commitment
    if commitment.has_active_commitment {
      if let Some(ac) = commitment.active_commitment {
        let _ = services
          .purchase
          .cart_commitment_add(AddCommitmentRequest {
            cart_id: f.cart_id.clone(),
            commitment_id: ac.commitment_id,
            discount_percentage: ac.discount_percentage,
          })
          .await
          .map_err(|e| ApiError::from(e))?
          .into_inner();
      }
    }
  }
  //
  // Commitment END
  //

  // Add customer
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
  // First query cart
  let cart: CartForm = services
    .purchase
    .cart_get_by_id(CartByIdRequest {
      cart_id: f.cart_id.clone(),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  // Check if we have commitment
  // If we have, remove it
  if cart.commitment_id.len() > 0 {
    // TODO! Manage error?
    let _ = services
      .purchase
      .cart_commitment_remove(RemoveCommitmentRequest {
        cart_id: cart.id,
        commitment_id: cart.commitment_id,
      })
      .await;
  }

  // Finally remove customer
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

  // Query SKU
  let sku = services
    .product
    .get_sku(GetSkuRequest {
      sku_id: upl_obj.sku_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Query Product
  let product = services
    .product
    .get_product(GetProductRequest {
      product_id: upl_obj.product_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  // Create UPL info object
  let upl_info_object = UplInfoObject {
    upl_id: upl_obj.id.clone(),
    name: match upl_obj
      .kind
      .clone()
      .ok_or(ApiError::internal_error("NO UPL KIND"))?
    {
      upl_obj::Kind::Sku(_) => sku.display_name,
      upl_obj::Kind::BulkSku(_) => sku.display_name,
      upl_obj::Kind::OpenedSku(opened_sku) => format!(
        "{} {}, {} {}",
        product.name,
        sku.subname,
        opened_sku.amount,
        upl_obj.product_unit.clone()
      ),
      upl_obj::Kind::DerivedProduct(derived_product) => format!(
        "{} {}, {} {}",
        product.name,
        sku.subname,
        derived_product.amount,
        upl_obj.product_unit.clone()
      ),
    },
    retail_net_price: upl_obj.price_net,
    vat: upl_obj.vat.clone(),
    retail_gross_price: upl_obj.price_gross,
    procurement_net_price: upl_obj.procurement_net_price,
    best_before: upl_obj.best_before.clone(),
    depreciated: upl_obj.depreciation.is_some(),
    upl_kind: Some(
      match upl_obj
        .kind
        .ok_or(ApiError::internal_error("NO UPL KIND"))?
      {
        proto::upl::upl_obj::Kind::Sku(_sku) => {
          proto::purchase::upl_info_object::UplKind::Sku(UplKindSku {
            sku: _sku.sku,
            piece: 1,
          })
        }
        proto::upl::upl_obj::Kind::BulkSku(_bulk_sku) => {
          proto::purchase::upl_info_object::UplKind::Sku(UplKindSku {
            sku: _bulk_sku.sku,
            piece: _bulk_sku.upl_pieces,
          })
        }
        proto::upl::upl_obj::Kind::OpenedSku(_opened_sku) => {
          proto::purchase::upl_info_object::UplKind::OpenedSku(UplKindOpenedSku {
            product_id: upl_obj.product_id,
            amount: _opened_sku.amount,
          })
        }
        proto::upl::upl_obj::Kind::DerivedProduct(_derived_product) => {
          proto::purchase::upl_info_object::UplKind::OpenedSku(UplKindOpenedSku {
            product_id: upl_obj.product_id,
            amount: _derived_product.amount,
          })
        }
      },
    ),
  };

  // Try add UPL InfoObject to Cart
  let cart: CartForm = services
    .purchase
    .cart_add_upl(proto::purchase::CartAddUplRequest {
      cart_id: cart_obj.id.clone(),
      upl: Some(upl_info_object),
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

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

  // Create invoice if needed
  if cart_closed.need_invoice {
    // Query purchase
    let purchase: PurchaseForm = services
      .purchase
      .purchase_get_by_id(PurchaseByIdRequest {
        purchase_id: cart_closed.id,
      })
      .await
      .map_err(|e| ApiError::from(e))?
      .into_inner()
      .into();

    // Convert purchase form into invoice request
    let invoice_request: InvoiceForm = purchase.clone().into();

    // Try start invoice creation as bg task
    let invoice_data = services
      .invoice
      .create_new(invoice_request)
      .await
      .map_err(|e| ApiError::from(e))?
      .into_inner();

    // Set invoice internal ID to purchase
    services
      .purchase
      .purchase_set_invoice_id(PurchaseSetInvoiceIdRequest {
        purchase_id: purchase.purchase_id,
        invoice_id: invoice_data.id,
      })
      .await
      .map_err(|e| ApiError::from(e))?;
  }

  // Return nothing if success
  Ok(reply::json(&""))
}

pub async fn cart_set_need_invoice(
  uid: u32,
  mut services: Services,
  f: CartSetInvoiceForm,
) -> ApiResult {
  // Check if cart valid
  let res: CartForm = services
    .purchase
    .cart_set_document(CartSetDocumentRequest {
      cart_id: f.cart_id,
      document_kind: match f.need_invoice {
        true => DocumentKind::Invoice,
        false => DocumentKind::Receipt,
      } as i32,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .try_into()?;

  Ok(reply::json(&res))
}
