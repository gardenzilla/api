use crate::{prelude::*, services::Services};
use gzlib::proto::{
  self,
  invoice::{ByIdRequest, DownloadRequest, DownloadResponse},
};
use proto::invoice::invoice_client::*;
use serde::{Deserialize, Serialize};
use tonic::transport::Channel;
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceDataForm {
  id: String,
  purchase_id: String,
  invoice_id: String,
  has_error: bool,
  created_by: u32,
  created_at: String,
}

impl From<proto::invoice::InvoiceData> for InvoiceDataForm {
  fn from(f: proto::invoice::InvoiceData) -> Self {
    Self {
      id: f.id,
      purchase_id: f.purchase_id,
      invoice_id: f.invoice_id,
      has_error: f.has_error,
      created_by: f.created_by,
      created_at: f.created_at,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoiceDownloadForm {
  invoice_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvoicePdfForm {
  pdf_base64: String,
}

impl From<DownloadResponse> for InvoicePdfForm {
  fn from(f: DownloadResponse) -> Self {
    Self {
      pdf_base64: f.pdf_base64,
    }
  }
}

pub async fn get_invoice_data(
  internal_invoice_id: String,
  uid: u32,
  mut services: Services,
) -> ApiResult {
  let res: InvoiceDataForm = services
    .invoice
    .get_by_id(ByIdRequest {
      id: internal_invoice_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}

pub async fn download(uid: u32, mut services: Services, f: InvoiceDownloadForm) -> ApiResult {
  let res: InvoicePdfForm = services
    .invoice
    .download(DownloadRequest {
      invoice_id: f.invoice_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(reply::json(&res))
}
