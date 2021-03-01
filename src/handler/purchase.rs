use std::{collections::HashMap, convert::TryInto};

use crate::{prelude::*, services::Services};
use chrono::{DateTime, Utc};
use gzlib::proto::{
  self,
  commitment::PurchaseInfo,
  invoice::{
    invoice_form::{Customer, Item, PaymentKind},
    InvoiceForm,
  },
  latex::Content,
  purchase::{purchase_object::ItemKind, PurchaseByIdRequest, PurchaseInfoObject, PurchaseObject},
};
use serde::{Deserialize, Serialize};
use warp::reply;

use super::cart::{PaymentForm, PaymentKindForm, UplInfoForm};

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseInfoForm {
  purchase_id: String,
  customer: Option<CustomerForm>,
  upl_count: u32,
  total_net_price: u32,
  total_vat: u32,
  total_gross_price: u32,
  balance: i32,
  payable: i32,
  document_invoice: bool,
  invoice_id: String,
  date_completion: String,
  payment_duedate: String,
  payment_expired: bool,
  profit_net: i32,
  restored: bool,
  created_by: u32,
  created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomerForm {
  id: u32,
  name: String,
  zip: String,
  location: String,
  street: String,
  tax_number: String,
}

impl From<PurchaseInfoObject> for PurchaseInfoForm {
  fn from(f: PurchaseInfoObject) -> Self {
    Self {
      purchase_id: f.purchase_id,
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
      upl_count: f.upl_count,
      total_net_price: f.total_net_price,
      total_vat: f.total_vat,
      total_gross_price: f.total_gross_price,
      balance: f.balance,
      payable: f.payable,
      document_invoice: f.document_invoice,
      invoice_id: f.invoice_id,
      date_completion: f.date_completion,
      payment_duedate: f.payment_duedate,
      payment_expired: f.payment_expired,
      profit_net: f.profit_net,
      restored: f.restored,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ItemKindForm {
  Sku,
  DerivedProduct,
  DepreciatedSku,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemForm {
  pub kind: ItemKindForm,
  pub product_id: u32,
  pub name: String,
  pub piece: u32,
  pub retail_price_net: u32,
  pub vat: String,
  pub retail_price_gross: u32,
  pub total_retail_price_net: u32,
  pub total_retail_price_gross: u32,
  pub upl_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoyaltyTransaction {
  pub loyalty_account_id: String,
  pub transaction_id: String,
  pub burned_points: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoyaltyCard {
  account_id: String,
  card_id: String,
  loyalty_level: String,
  balance_opening: i32,
  burned_points: i32,
  earned_points: i32,
  balance_closing: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PurchaseForm {
  pub purchase_id: String,
  pub customer: Option<CustomerForm>,
  pub commitment_id: String,
  pub commitment_discount_percentage: u32,
  pub commitment_discount_amount_gross: u32,
  pub loyalty_card: Option<LoyaltyCard>,
  pub burned_loyalty_points: u32,
  pub items: Vec<ItemForm>,
  pub upl_info_objects: Vec<UplInfoForm>,
  pub need_invoice: bool,
  pub invoice_id: String,
  pub total_net_price: u32,
  pub total_vat: u32,
  pub total_gross_price: u32,
  pub payment_kind: PaymentKindForm,
  pub payments: Vec<PaymentForm>,
  pub burned_points: Vec<LoyaltyTransaction>,
  pub payable: i32,
  pub payment_balance: i32,
  pub date_completion: String,
  pub payment_duedate: String,
  pub profit_net: i32,
  pub restored: bool,
  pub owner_uid: u32,
  pub created_by: u32,
  pub created_at: String,
}

impl From<PurchaseObject> for PurchaseForm {
  fn from(f: PurchaseObject) -> Self {
    let p: proto::purchase::PaymentKind =
      proto::purchase::PaymentKind::from_i32(f.payment_kind).unwrap(); // TODO! FIX IT

    Self {
      purchase_id: f.id,
      customer: match f.customer.clone() {
        Some(c) => Some(CustomerForm {
          id: c.customer_id,
          name: c.name,
          zip: c.zip,
          location: c.location,
          street: c.street,
          tax_number: c.tax_number,
        }),
        None => None,
      },
      commitment_id: f.commitment_id,
      commitment_discount_percentage: f.commitment_discount_percentage,
      commitment_discount_amount_gross: f.commitment_discount_amount_gross,
      loyalty_card: match f.loyalty_card {
        Some(lc) => Some(LoyaltyCard {
          account_id: lc.account_id,
          card_id: lc.card_id,
          loyalty_level: lc.loyalty_level,
          balance_opening: lc.balance_opening,
          burned_points: lc.burned_points,
          earned_points: lc.earned_points,
          balance_closing: lc.balance_closing,
        }),
        None => None,
      },
      burned_loyalty_points: f.burned_loyalty_points,
      items: f
        .items
        .iter()
        .map(|i| {
          let kind: ItemKind = ItemKind::from_i32(i.kind).unwrap(); // TODO! FIX IT

          ItemForm {
            kind: match kind {
              ItemKind::Sku => ItemKindForm::Sku,
              ItemKind::DerivedProduct => ItemKindForm::DerivedProduct,
              ItemKind::DepreciatedSku => ItemKindForm::DepreciatedSku,
            },
            product_id: i.product_id,
            name: i.name.clone(),
            piece: i.piece,
            retail_price_net: i.retail_price_net,
            vat: i.vat.clone(),
            retail_price_gross: i.retail_price_gross,
            total_retail_price_net: i.total_retail_price_net,
            total_retail_price_gross: i.total_retail_price_gross,
            upl_ids: i.upl_ids.clone(),
          }
        })
        .collect(),
      upl_info_objects: f
        .upl_info_objects
        .iter()
        .map(|u| u.clone().into())
        .collect(),
      need_invoice: f.need_invoice,
      invoice_id: f.invoice_id,
      total_net_price: f.total_net,
      total_vat: f.total_vat,
      total_gross_price: f.total_gross,
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
      burned_points: f
        .burned_points
        .into_iter()
        .map(|tr| LoyaltyTransaction {
          loyalty_account_id: tr.loyalty_account_id,
          transaction_id: tr.transaction_id,
          burned_points: tr.burned_points,
        })
        .collect::<Vec<LoyaltyTransaction>>(),
      payable: f.payable,
      payment_balance: f.payment_balance,
      date_completion: f.date_completion,
      payment_duedate: f.payment_duedate,
      profit_net: f.profit_net,
      restored: f.restored,
      owner_uid: f.owner_uid,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

impl From<PurchaseForm> for InvoiceForm {
  fn from(f: PurchaseForm) -> Self {
    let mut items: Vec<Item> = f
      .items
      .iter()
      .map(|i| Item {
        name: i.name.clone(),
        quantity: i.piece as i32,
        unit: "db".to_string(),
        price_unit_net: i.retail_price_net as i32,
        vat: i.vat.clone(),
        total_price_net: i.total_retail_price_net as i32,
        total_price_vat: i.total_retail_price_gross as i32 - i.total_retail_price_net as i32, // TODO! Fix it
        total_price_gross: i.total_retail_price_gross as i32,
        comment: "".to_string(),
      })
      .collect();

    // Insert commitment discount if have one
    if f.commitment_id.len() > 0 {
      let gross = (f.commitment_discount_amount_gross as i32) * -1;
      let net = (gross as f32 / 1.27).round() as i32;
      let vat = gross - net;
      items.push(Item {
        name: format!("Egyedi kedvezmény ({}%)", f.commitment_discount_percentage),
        quantity: 1,
        unit: "db".to_string(),
        price_unit_net: net,
        vat: "27".to_string(),
        total_price_net: net,
        total_price_vat: vat,
        total_price_gross: gross,
        comment: format!("Commitment azonosító: {}", f.commitment_id.clone()),
      });
    }

    // Insert burned points discount if have one
    if let Some(loyalty) = f.loyalty_card {
      // Only if we burned points
      if loyalty.burned_points != 0 {
        let gross = (loyalty.burned_points as i32) * -1;
        let net = (gross as f32 / 1.27).round() as i32;
        let vat = gross - net;
        items.push(Item {
          name: "Törzsvásárlói kedvezmény".to_string(),
          quantity: 1,
          unit: "db".to_string(),
          price_unit_net: net,
          vat: "27".to_string(),
          total_price_net: net,
          total_price_vat: vat,
          total_price_gross: gross,
          comment: format!(
            "Törzsvásárlói fiók azonosító: {}",
            loyalty.account_id.clone()
          ),
        });
      }
    }

    Self {
      purchase_id: f.purchase_id,
      customer: Some(Customer {
        id: match f.customer.clone() {
          Some(c) => c.id,
          None => 0,
        },
        name: match f.customer.clone() {
          Some(c) => c.name,
          None => String::default(),
        },
        tax_number: match f.customer.clone() {
          Some(c) => c.tax_number,
          None => String::default(),
        },
        zip: match f.customer.clone() {
          Some(c) => c.zip,
          None => String::default(),
        },
        location: match f.customer.clone() {
          Some(c) => c.location,
          None => String::default(),
        },
        street: match f.customer.clone() {
          Some(c) => c.street,
          None => String::default(),
        },
        email: "".to_owned(),
      }),
      items,
      payment_kind: match f.payment_kind {
        PaymentKindForm::Cash => PaymentKind::Cash,
        PaymentKindForm::Card => PaymentKind::Card,
        PaymentKindForm::Transfer => PaymentKind::Transfer,
      } as i32,
      payment_duedate: f.payment_duedate,
      date: Utc::now().to_rfc3339(),
      completion_date: f.date_completion,
      total_net: f.total_net_price as i32,
      total_vat: f.total_vat as i32,
      total_gross: f.total_gross_price as i32,
      created_by: f.created_by,
    }
  }
}

impl From<PurchaseForm> for PurchaseInfoForm {
  fn from(f: PurchaseForm) -> Self {
    Self {
      purchase_id: f.purchase_id,
      customer: f.customer,
      upl_count: f.upl_info_objects.len() as u32,
      total_net_price: f.total_net_price,
      total_vat: f.total_vat,
      total_gross_price: f.total_gross_price,
      balance: f.payment_balance,
      payable: f.payable,
      document_invoice: f.need_invoice,
      invoice_id: f.invoice_id,
      date_completion: f.date_completion,
      payment_duedate: f.payment_duedate.clone(),
      payment_expired: DateTime::parse_from_rfc3339(&f.payment_duedate).unwrap() > Utc::now(),
      profit_net: f.profit_net,
      restored: f.restored,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PurchaseIdForm {
  purchase_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PdfBase64Form {
  pdf_base64: String,
}

pub async fn purchase_info_get_by_id(
  _uid: u32,
  mut services: Services,
  f: PurchaseIdForm,
) -> ApiResult {
  let res: PurchaseForm = services
    .purchase
    .purchase_get_by_id(PurchaseByIdRequest {
      purchase_id: f.purchase_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  let info: PurchaseInfoForm = res.into();
  Ok(reply::json(&info))
}

pub async fn purchase_get_by_id(
  purchase_id: String,
  _uid: u32,
  mut services: Services,
) -> ApiResult {
  let res: PurchaseForm = services
    .purchase
    .purchase_get_by_id(PurchaseByIdRequest { purchase_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn purchase_get_all(_uid: u32, mut services: Services) -> ApiResult {
  let res: Vec<String> = services
    .purchase
    .purchase_get_all(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .purchase_ids;
  Ok(reply::json(&res))
}

pub async fn get_bulk(_uid: u32, mut services: Services, purchase_ids: Vec<String>) -> ApiResult {
  let mut all = services
    .purchase
    .purchase_get_info_bulk(proto::purchase::PurchaseBulkRequest { purchase_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  let mut result: Vec<PurchaseInfoForm> = Vec::new();
  while let Some(pinfo) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(pinfo.into());
  }
  Ok(reply::json(&result))
}

pub async fn get_receipt(_uid: u32, mut services: Services, f: PurchaseIdForm) -> ApiResult {
  let res: PurchaseForm = services
    .purchase
    .purchase_get_by_id(PurchaseByIdRequest {
      purchase_id: f.purchase_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  let receipt = crate::receipt::Receipt::new(
    res.purchase_id,
    res
      .items
      .iter()
      .map(|i| crate::receipt::Item {
        sku: "-".to_string(),
        name: i.name.clone(),
        piece: i.piece,
        gross_price_total: i.total_retail_price_gross,
      })
      .collect(),
    res.total_gross_price as i32,
    res.commitment_discount_amount_gross as i32,
    res.commitment_discount_percentage,
    res.loyalty_card.is_some(),
    match res.loyalty_card.clone() {
      Some(lc) => lc.card_id,
      None => "".to_string(),
    },
    match res.loyalty_card.clone() {
      Some(lc) => lc.burned_points,
      None => 0,
    },
    match res.loyalty_card.clone() {
      Some(lc) => lc.earned_points,
      None => 0,
    },
    match res.loyalty_card.clone() {
      Some(lc) => lc.loyalty_level,
      None => "".to_string(),
    },
    match res.loyalty_card.clone() {
      Some(lc) => lc.balance_opening,
      None => 0,
    },
    match res.loyalty_card.clone() {
      Some(lc) => lc.balance_closing,
      None => 0,
    },
    res.total_gross_price as i32,
    DateTime::parse_from_rfc3339(&res.created_at)
      .unwrap()
      .with_timezone(&Utc),
  );

  let template = receipt.to_latex();

  let icon_bytes = include_bytes!("../../static/icon.jpg");

  // Call latex service
  let result = services
    .latex
    .process(Content {
      main_latex_file: template.as_bytes().to_owned(),
      attachments: {
        let mut files: HashMap<String, Vec<u8>> = HashMap::new();
        files.insert("logo.jpg".to_string(), icon_bytes.to_vec());
        files
      },
    })
    .await
    .map_err(|e| ApiError::bad_request("Hiba a latex szerviztől"))?
    .into_inner();

  let res: PdfBase64Form = PdfBase64Form {
    pdf_base64: base64::encode(result.content),
  };

  Ok(reply::json(&res))
}
