use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Notebook {
    /// Path to the notebook's SQLite database
    pub dbpath: Option<String>,

    /// Notebook's cell data
    pub cells: Vec<Cell>,
}

impl Notebook {
    /// Create a new notebook instance.
    pub fn new() -> Notebook {
        Notebook {
            dbpath: None,
            cells: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Cell {
    #[serde(rename = "query")]
    Query(QueryCell),

    #[serde(rename = "markdown")]
    Markdown(MarkdownCell),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MarkdownCell {
    /// The cell's markdown contents
    pub contents: String,
}

impl MarkdownCell {
    /// Create a new markdown cell.
    pub fn new() -> MarkdownCell {
        MarkdownCell {
            ..Default::default()
        }
    }

    /// Set the cell's markdown contents.
    pub fn with_content(&mut self, content: String) -> &mut MarkdownCell {
        self.contents = content;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryCell {
    /// The cell's query contents
    pub query: String,

    /// The cell's query results
    /// 
    /// This is a vector of rows, where each row is a map of column 
    /// name to value. The value is an `Option<String>` because the 
    /// value may be `NULL` and because it has already been formatted
    /// as a string (except for nulls).
    pub results: Option<Vec<HashMap<String, Option<String>>>>,

    /// Amount of time, in milliseconds, that the
    /// query took to execute.
    pub query_time: Option<u128>,
}

impl QueryCell {
    /// Create a new query cell.
    pub fn new() -> QueryCell {
        QueryCell {
            ..Default::default()
        }
    }

    /// Set the query contents of the cell.
    pub fn with_query(&mut self, query: String) -> &mut QueryCell {
        self.query = query;
        self
    }
}

