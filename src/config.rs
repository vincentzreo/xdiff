use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use url::Url;

use crate::ExtraArgs;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    pub res: ResponseProfile,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,
    pub url: Url,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,
    #[serde(
        with = "http_serde::header_map",
        skip_serializing_if = "HeaderMap::is_empty",
        default
    )]
    pub headers: HeaderMap,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

impl DiffConfig {
    pub async fn load_yaml(path: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yaml(&content)
    }
    pub fn from_yaml(content: &str) -> anyhow::Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }
    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

impl DiffProfile {
    pub async fn diff(&self, args: ExtraArgs) -> anyhow::Result<String> {
        // let res1 = self.req1.send(&args).await?;
        // let res2 = self.req2.send(&args).await?;

        // let text1 = res1.filter_text(&self.res).await?;
        // let text2 = res2.filter_text(&self.res).await?;

        // text_diff(&text1, &text2);
        println!("profile: {:?}", self);
        println!("args: {:?}", args);
        Ok("".to_string())
    }
}
