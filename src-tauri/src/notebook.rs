use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Notebook {
    /// Path to the notebook file
    pub filepath: String,

    /// SQLite database file path
    pub dbpath: String,

    /// Notebook cells
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Cell {
    #[serde(rename = "query")]
    Query(QueryCell),

    #[serde(rename = "markdown")]
    Markdown(MarkdownCell),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarkdownCell {
    /// Markdown content
    pub content: String,
}

#[derive(Debug, Clone, Default,  Serialize, Deserialize)]
pub struct QueryCell {
    /// Query content
    pub query: String,

    /// Query results (if run)
    pub results: Option<HashMap<String, Vec<Value>>>,

    /// How long did the query take to run?
    #[serde(rename = "queryTime")]
    pub query_time: Option<f64>,

    /// Error returned from query, if any
    #[serde(rename = "queryError")]
    pub query_error: Option<String>,
}

