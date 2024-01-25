use serde::{Serialize, Deserialize};


/// Contains Table information
#[derive(Serialize, Deserialize)]
pub struct Table {
    pub table: TableData,
    pub meta: TableMetaData,
}

/// Intermediate representation of a table content.
#[derive(Serialize, Deserialize)]
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub width: usize,
    pub height: usize,
}

/// Intermediate representation of a table metadata.
#[derive(Serialize, Deserialize)]
pub struct TableMetaData {
    pub caption: String,
    pub label: String,
}

impl Default for Table {
    // initialize with default values
    fn default() -> Self {
        Self {
            table: TableData::default(),
            meta: TableMetaData::default(),
        }
    }
}

impl Default for TableData {
    // initialize with default values
    fn default() -> Self {
        Self {
            headers: vec![],
            rows: vec![],
            width: 0,
            height: 0,
        }
    }
}

impl Default for TableMetaData {
    // initialize with default values
    fn default() -> Self {
        Self {
            caption: "".to_string(),
            label: "".to_string(),
        }
    }
}
