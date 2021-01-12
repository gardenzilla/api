use crate::{prelude::*, services::Services};
use gzlib::proto::source::{CreateSourceRequest, GetSourceRequest, SourceObject};
use serde::{Deserialize, Serialize};
use warp::reply;

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceForm {
  source_id: u32,
  name: String,
  address: String,
  email: Vec<String>,
  phone: Vec<String>,
  created_at: String,
  created_by: u32,
}

impl From<SourceObject> for SourceForm {
  fn from(so: SourceObject) -> Self {
    Self {
      source_id: so.id,
      name: so.name,
      address: so.address,
      email: so.email,
      phone: so.phone,
      created_at: so.created_at,
      created_by: so.created_by,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewSourceForm {
  name: String,
  address: String,
  email: Vec<String>,
  phone: Vec<String>,
}

pub async fn create_new(uid: u32, mut services: Services, f: NewSourceForm) -> ApiResult {
  let res: SourceForm = services
    .source
    .create_source(CreateSourceRequest {
      name: f.name,
      address: f.address,
      email: f.email,
      phone: f.phone,
      created_by: uid,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn get_by_id(source_id: u32, _uid: u32, mut services: Services) -> ApiResult {
  let res: SourceForm = services
    .source
    .get_source(GetSourceRequest { source_id })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn update(source_id: u32, _uid: u32, mut services: Services, f: SourceForm) -> ApiResult {
  let res: SourceForm = services
    .source
    .update_source(SourceObject {
      id: source_id,
      name: f.name,
      address: f.address,
      email: f.email,
      phone: f.phone,
      created_at: f.created_at,
      created_by: f.created_by,
    })
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner()
    .into();
  Ok(reply::json(&res))
}

pub async fn get_all(_uid: u32, mut services: Services) -> ApiResult {
  let mut result: Vec<SourceForm> = Vec::new();

  let mut all = services
    .source
    .get_all_sources(())
    .await
    .map_err(|e| ApiError::from(e))?
    .into_inner();

  while let Some(so) = all.message().await.map_err(|e| ApiError::from(e))? {
    result.push(so.into());
  }

  Ok(warp::reply::json(&result))
}
