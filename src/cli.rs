use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::ExtraArgs;

/// Diff two http requests and compare the diferience of responses
#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand, Clone)]
#[non_exhaustive]
pub enum Action {
    /// DIFF two API responses based on given profile
    Run(RunArgs),
}

#[derive(Debug, Parser, Clone)]
pub struct RunArgs {
    /// profile name
    #[clap(short, long, value_parser)]
    pub profile: String,
    /// overrides args. Could be used to override the query, headers and body of request.
    /// For query params, use `-e key=value`,
    /// For headers, use `-e %key=value`,
    /// For body, ues `-e @key=value`
    #[clap(short, long, value_parser=parse_key_val, num_args=1)]
    pub extra_params: Vec<KeyVal>,
    /// configuration to ues
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVal {
    key_type: KeyValType,
    key: String,
    value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, "=");
    let key = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
        .trim();
    let value = parts
        .next()
        .ok_or_else(|| anyhow!("Invalid key value pair: {}", s))?
        .trim();
    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key value pair")),
    };
    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string(),
    })
}

impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(value: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];
        for arg in value {
            match arg.key_type {
                KeyValType::Body => body.push((arg.key, arg.value)),
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
            }
        }
        Self {
            headers,
            query,
            body,
        }
    }
}
