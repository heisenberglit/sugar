use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::ValidateParserError;
use crate::{common::*, validate::parser};

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Metadata {
    pub name: String,
    pub symbol: Option<String>,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_fee_basis_points: Option<u16>,
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    pub attributes: Vec<Attribute>,
    pub properties: Property,
}

impl Metadata {
    pub fn validate(&mut self) -> Result<(), ValidateParserError> {
        parser::check_name(&self.name)?;
        parser::check_url(&self.image)?;

        // If users are using the old format, we do validation on those values.
        if let Some(sfbp) = &self.seller_fee_basis_points {
            parser::check_seller_fee_basis_points(*sfbp)?;
        }
        if let Some(symbol) = &self.symbol {
            parser::check_symbol(symbol)?;
        }

        if let Some(creators) = &self.properties.creators {
            parser::check_creators_shares(creators)?;
            parser::check_creators_addresses(creators)?;
        }

        if self.properties.category.is_none() {
            let category = match &self.animation_url {
                Some(_) => "video",
                None => "image",
            };
            self.properties.category = Some(category.to_string());

            println!(
                "{} missing `properties.creator` for nft {}, defaulting to {}",
                WARNING_EMOJI, &self.name, category
            );
        }
        parser::check_category(
            self.properties
                .category
                .as_ref()
                .expect("unreachable, should never throw"),
        )?;

        if let Some(animation_url) = &self.animation_url {
            parser::check_url(animation_url)?;
        }

        if let Some(external_url) = &self.external_url {
            parser::check_url(external_url)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Property {
    pub files: Vec<FileAttr>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<Creator>>,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Creator {
    pub address: String,
    pub share: u16,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct FileAttr {
    pub uri: String,
    #[serde(rename = "type")]
    pub file_type: String,
}
