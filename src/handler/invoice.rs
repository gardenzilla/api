use std::collections::HashMap;

use crate::{prelude::*, services::Services};
use gzlib::proto::{
  self,
  invoice::{ByIdRequest, DownloadRequest, DownloadResponse},
  latex::Content,
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
  let pdf_base64: String = services
    .invoice
    .download(DownloadRequest {
      invoice_id: f.invoice_id,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .pdf_base64;

  let template = r#"
    \documentclass{standalone}
    \usepackage{graphicx}
    \usepackage{tabto}
    \usepackage[utf8]{inputenc}
    \usepackage[T1]{fontenc}
    \usepackage{pdfpages}
    
    \begin{document}
      
      \begin{minipage}[left]{7cm}
        \centering
          \vspace{0.3cm}
          \includegraphics[width=50px]{icon.jpg} \\
          \Huge{\textbf{GardenZilla}} \\
          \vspace{0.2cm}
          \normalsize{\textmd{Kert és Otthon}}\\
          \vspace{0cm}
          
          \hspace*{-0.5cm}\includegraphics[]{invoice.pdf}
      \end{minipage}
      
      
    \end{document}"#;

  let icon_bytes = include_bytes!("../../static/icon.jpg");

  let invoice_bytes = base64::decode(pdf_base64).expect("Error while decoding B64 document");

  // Call latex service
  let result = services
    .latex
    .process(Content {
      main_latex_file: template.as_bytes().to_owned(),
      attachments: {
        let mut files: HashMap<String, Vec<u8>> = HashMap::new();
        files.insert("icon.jpg".to_string(), icon_bytes.to_vec());
        files.insert("invoice.pdf".to_string(), invoice_bytes);
        files
      },
    })
    .await
    .map_err(|e| ApiError::bad_request("Hiba a latex szerviztől"))?
    .into_inner();

  let res: InvoicePdfForm = InvoicePdfForm {
    pdf_base64: base64::encode(result.content),
  };

  Ok(reply::json(&res))
}
