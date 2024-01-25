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

impl Table {
    /// Generate dummy table for testing
    pub fn dummy() -> Self {
        let mut table = Table::default();
        table.table.headers = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        table.table.rows = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
            vec!["7".to_string(), "8".to_string(), "9".to_string()],
        ];
        table.table.width = 3;
        table.table.height = 3;
        table.meta.caption = "Dummy table".to_string();
        table.meta.label = "dummy".to_string();
        table
    }
}