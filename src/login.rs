// Copyright (C) 2020 Peter Mezei
//
// This file is part of Gardenzilla.
//
// Gardenzilla is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// Gardenzilla is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Gardenzilla.  If not, see <http://www.gnu.org/licenses/>.

use crypto::sha2::Sha256;
use jwt::{Header, Token};
use serde::{Deserialize, Serialize};
use std::default::Default;

pub struct UserId(String);

impl UserId {
    fn new(uid: String) -> Self {
        UserId(uid)
    }
}

impl std::ops::Deref for UserId {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Deserialize, Serialize, RustcDecodable, RustcEncodable)]
struct Custom {
    uid: String,
    rhino: bool,
}

pub enum LoginError {
    InternalError,
    WrongToken,
}

pub type LoginResult<T> = Result<T, LoginError>;

pub fn create_token(user_id: &str) -> LoginResult<String> {
    let header: Header = Default::default();
    let claims = Custom {
        uid: user_id.into(),
        rhino: true,
        ..Default::default()
    };
    let token = Token::new(header, claims);

    match token.signed(b"secret_key", Sha256::new()) {
        Ok(token) => return Ok(token),
        Err(_) => return Err(LoginError::InternalError),
    }
}

pub fn verify_token(token: &str) -> LoginResult<UserId> {
    let token = match Token::<Header, Custom>::parse(token) {
        Ok(v) => v,
        Err(_) => return Err(LoginError::WrongToken),
    };

    if token.verify(b"secret_key", Sha256::new()) {
        Ok(UserId::new(token.claims.uid))
    } else {
        Err(LoginError::WrongToken)
    }
}
