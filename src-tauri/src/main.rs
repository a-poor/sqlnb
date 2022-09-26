#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use std::collections::HashMap;

mod notebook;


#[derive(Default)]
struct ClientConnection(Mutex<Option<Client>>);

#[derive(Default)]
pub struct Client {
    /// Map from notebook file path to notebook data
    notebooks: HashMap<String, notebook::Notebook>,

    /// Map from notebook file path to SQLite db connection
    connections: HashMap<String, sqlite::Connection>,
}

impl Client {
    /// Create a new client
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new notebook
    pub fn add_notebook(&mut self, nbpath: String, dbpath: String) {
        self.notebooks.insert(
            nbpath.clone(), 
            notebook::Notebook {
                filepath: nbpath.clone(),
                dbpath: dbpath.clone(),
                cells: vec![],
            }
        );
    }

    /// Add a database connection
    pub fn add_connection(&mut self, nbpath: String, conn: sqlite::Connection) {
        self.connections.insert(nbpath, conn);
    }

    /// Add a new cell to a notebook
    pub fn add_cell(&mut self, nbpath: String, cell: notebook::Cell) {
        if let Some(nb) = self.notebooks.get_mut(&nbpath) {
            nb.cells.push(cell);
        }
    }

    /// Get a notebook
    pub fn get_notebook(&self, nbpath: &str) -> Option<&notebook::Notebook> {
        self.notebooks.get(nbpath)
    }

    /// Get a database connection or attempt to open one
    /// if it doesn't exist.
    pub fn get_connection(&mut self, nbpath: &str) -> Result<&sqlite::Connection, String> {
        // If the connection already exists, return it...
        if let Some(conn) = self.connections.get(nbpath) {
            return Ok(conn);
        }

        // Otherwise, try to get the notebook data...
        let nb = self
            .get_notebook(nbpath)
            .ok_or("Notebook not found")?;

        // Get the dbpath from the notebook data...
        let dbpath = nb.dbpath.clone();

        // Open the database connection...
        let conn = sqlite::open(dbpath)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // Add the connection to the client...
        self.add_connection(nbpath.to_string(), conn);

        // Return the connection...
        Ok(self.connections.get(nbpath).unwrap())
    }

    /// Run a query cell
    pub fn run_query_cell(&self, nbpath: &str, cell_idx: usize) -> Result<notebook::QueryCell, String> {
        // Get the notebook, if it exists...
        let nb = self.notebooks
            .get(nbpath)
            .ok_or("Notebook not found")?;

        // Get the database connection, if it exists...
        let conn = self.get_connection(nbpath)?;

        // Get the query cell, if it exists...
        let cell = nb.cells
            .get(cell_idx)
            .ok_or("Cell not found")?;

        // Make sure the cell is a query cell...
        let cell = match cell {
            notebook::Cell::Query(cell) => cell,
            _ => return Err("Cell is not a query cell".to_string()),
        };

        // Run the query...
        let results = conn
            .execute(cell.query.clone());




        // Check for the notebook in the client's notebooks...
        if let Some(nb) = self.notebooks.get(nbpath) {

            // Check for the DB connection in the client's connections...
            if let Some(conn) = self.connections.get(nbpath) {

                if let Some(notebook::Cell::Query(cell)) = nb.cells.get(cell_idx) {
                    let mut stmt = conn.prepare(&cell.query).map_err(|e| e.to_string())?;
                    let mut results = HashMap::new();
                    let mut row = 0;
                    while let sqlite::State::Row = stmt.next().map_err(|e| e.to_string())? {
                        let mut row_results = Vec::new();
                        for col in 0..stmt.column_count() {
                            row_results.push(stmt.read::<serde_json::Value>(col).map_err(|e| e.to_string())?);
                        }
                        results.insert(row.to_string(), row_results);
                        row += 1;
                    }
                    Ok(())
                } else {
                    Err("Cell is not a query cell".to_string())
                }
            } else {
                Err("No connection for notebook".to_string())
            }
        } else {
            Err("No notebook found".to_string())
        }
    }

}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(ClientConnection(Default::default()))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
