use crate::{prelude::*, services::Services};
use bytes::{Buf, BufMut};
use futures_util::TryStreamExt;
use gzlib::proto::{
  sku_image::{CoverBulkRequest, CoverObj, NewRequest, SkuObj, SkuRequest},
  sku_image_processer::AddRequest,
};
use serde::{Deserialize, Serialize};
use warp::{multipart, Filter};

use warp::{
  multipart::{FormData, Part},
  reply,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkuImageForm {
  sku: u32,
  cover_image_id: String,
  image_ids: Vec<String>,
}

impl From<SkuObj> for SkuImageForm {
  fn from(f: SkuObj) -> Self {
    Self {
      sku: f.sku,
      cover_image_id: f.cover_image_id,
      image_ids: f.image_ids,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoverForm {
  sku: u32,
  cover_image_id: String,
}

impl From<CoverObj> for CoverForm {
  fn from(f: CoverObj) -> Self {
    Self {
      sku: f.sku,
      cover_image_id: f.cover_image_id,
    }
  }
}

pub async fn add_new(sku: u32, _uid: u32, mut services: Services, f: FormData) -> ApiResult {
  let parts: Vec<Part> = f.try_collect().await.map_err(|e| {
    eprintln!("form error: {}", e);
    warp::reject::reject()
  })?;

  for p in parts {
    if p.name() == "file" {
      let content_type = p.content_type();
      let file_extension;
      match content_type {
        Some(file_type) => match file_type {
          "application/jpg" | "application/jpeg" | "image/jpg" | "image/jpeg" => {
            file_extension = "jpg";
          }
          v => {
            eprintln!("invalid file type found: {}; ONLY JPG", v);
            return Err(warp::reject::reject());
          }
        },
        None => {
          eprintln!("file type could not be determined");
          return Err(warp::reject::reject());
        }
      }

      let file_name = match &p.filename() {
        Some(file_n) => file_n.to_string(),
        None => "new_image_no_name".to_string(),
      };

      let image_bytes = p
        .stream()
        .try_fold(Vec::new(), |mut vec, mut data| {
          data.copy_to_slice(&mut vec);
          async move { Ok(vec) }
        })
        .await
        .map_err(|e| {
          eprintln!("reading file error: {}", e);
          warp::reject::reject()
        })?;

      // Try to create SKU image meta
      let image_id = services
        .sku_image
        .add_new(NewRequest {
          sku,
          file_name,
          file_extension: file_extension.to_string(),
          image_bytes: image_bytes.clone(),
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner()
        .new_image_id;

      // Try to send image to SKU IMAGE processer
      let _ = services
        .sku_img_processer
        .add_image(AddRequest {
          sku,
          image_id,
          image_bytes,
        })
        .await
        .map_err(|e| ApiError::from(e))?
        .into_inner();
    }
  }

  Ok(warp::reply::json(&()))
}

pub async fn get_images(sku: u32, _uid: u32, mut services: Services) -> ApiResult {
  let res: SkuImageForm = services
    .sku_image
    .get_images(SkuRequest { sku })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();

  Ok(warp::reply::json(&res))
}

pub async fn get_cover_bulk(_uid: u32, mut services: Services, sku_ids: Vec<u32>) -> ApiResult {
  let mut result: Vec<CoverForm> = Vec::new();

  let mut all = services
    .sku_image
    .get_cover_bulk(CoverBulkRequest { sku_ids })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  while let Some(so) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(so.into());
  }

  Ok(warp::reply::json(&result))
}
