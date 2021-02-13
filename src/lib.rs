pub mod auth;
pub mod interface;

use std::collections::BTreeMap;

use anyhow::Result;
use reqwest::blocking::get;
use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    SerdeQs(#[from] serde_qs::Error),
}

pub trait Request<T>: Sized
where
    T: DeserializeOwned,
{
    // fn endpoint(&self) -> EndPoints;
    // fn params(&self) -> Params;
    fn url(&self) -> Result<Url>;
    fn execute(self) -> Result<T> {
        let response = get(self.url()?)?.json()?;
        Ok(response)
    }
}

#[derive(Debug, Clone)]
pub struct Params<'a> {
    map: BTreeMap<&'a str, &'a str>,
}
