use std::fs;
use std::path::Path;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use anyhow::{Context, Result};


#[derive(Debug)]
pub struct Resources {
    pub main_js: String,
    pub modules_js: String,
    pub parent_mapping_json: String,
    pub out_html: String,
    pub index_html: String
}

impl Resources {
    pub fn new(main_js: String, modules_js: String, parent_mapping_json: String, out_html: String, index_html: String) -> Self {
        Resources {
            main_js,
            modules_js,
            parent_mapping_json,
            out_html,
            index_html
        }
    }
}