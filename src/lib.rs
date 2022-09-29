use std::collections::HashMap;
use serde_json::Value;
use serde::{Serialize, Deserialize};


pub struct Client {
    /// Notebooks the client is currently managing.
    /// 
    /// Maps notebook filepaths to notebook objects.
    pub notebooks: HashMap<String, Box<ActiveNotebook>>,

    /// A counter for creating new notebook IDs.
    /// 
    /// For example, a new notebook name may be of
    /// the form "notebook-{{ nb_inc }}.sql.nb"
    pub nb_inc: usize,
}

impl Client {
    /// Create a new instance of Client.
    pub fn new() -> Client {
        Client {
            notebooks: HashMap::new(),
            nb_inc: 0,
        }
    }

    /// Checks if a notebook with the given name exists
    /// in the client's notebook map.
    pub fn does_notebook_exist(&self, name: &str) -> bool {
        self.notebooks.contains_key(name)
    }

    /// Return a new notebook name, using the client's
    /// notebook counter.
    pub fn get_new_nb_name(&mut self) -> String {
        // Format the notebook name, using the counter.
        let nb_id = format!("notebook-{}.sql.nb", self.nb_inc);

        // Increment the counter.
        self.nb_inc += 1;

        // Return the notebook name.
        nb_id
    }
}

/// A notebook's save-state, on disk.
#[derive(PartialEq, Debug)]
pub enum SaveSate {
    /// The notebook has never been saved.
    NeverSaved,

    /// The notebook has been saved, but the client's
    /// in-memory copy is ahead of the on-disk copy.
    Dirty,

    /// The notebook has been saved, and the on-disk
    /// copy is up-to-date.
    Clean,
}

impl Default for SaveSate {
    fn default() -> Self {
        SaveSate::NeverSaved
    }
}

#[derive(Default)]
pub struct ActiveNotebook {
    /// Path to the notebook file.
    /// Also used as it's ID.
    pub nbpath: String,

    /// The notebook data as it is stored on disk
    pub data: Notebook,

    /// A connection to the notebook's SQLite database
    pub connection: Option<sqlite::Connection>,

    /// Should the notebook's connection to the
    /// database be read-only?
    pub db_is_read_only: bool,

    /// The relationship between the client's in-memory
    /// copy of the notebook and the on-disk copy.
    /// 
    /// Has the notebook never been saved? Is the
    /// in-memory copy ahead of the on-disk copy?
    /// Or is the on-disk copy up-to-date?
    /// 
    /// NOTE: Future iterations may want to track when the
    /// in-memory and on-disk representations of the notebook
    /// were last modified, so the client can warn the user if
    /// the on-disk copy is more up-to-date than the in-memory 
    /// copy (and offer to reload the notebook).
    pub save_state: SaveSate,
}

impl ActiveNotebook {
    /// Create a new instance of ActiveNotebook.
    pub fn new(nbpath: String) -> ActiveNotebook {
        ActiveNotebook {
            nbpath,
            ..Default::default()
        }
    }

    /// Attempts to open a connection to the notebook's
    /// SQLite database.
    /// 
    /// If the connection is opened successfully, the
    /// connection will be stored in the `.connection`
    /// field.
    /// 
    /// If the notebook doesn't have a database path, or
    /// if opening the connection fails, an error will be
    /// returned.
    /// 
    /// If this ActiveNotebook's `.db_is_read_only` field
    /// is set to true, the connection will be opened in
    /// read-only mode.
    /// 
    /// Additionally, the SQLite connection will be opened
    /// with the _create_ and the _full mutex_ `OpenFlag`.
    pub fn load_db_connection(&mut self) -> Result<(), String> {
        // If the notebook doesn't have a database path,
        // return an error.
        if self.data.dbpath.is_none() {
            return Err("Notebook has no database path".to_string());
        }

        // If the notebook already has a connection, noop.
        if self.connection.is_some() {
            return Ok(());
        }

        // Create the SQLite DB connection flags...
        let mut flags = sqlite::OpenFlags::new()
            .set_full_mutex()
            .set_create();

        // Set read-only-ness if needed...
        flags = if self.db_is_read_only {
            flags.set_read_only()
        } else {
            flags.set_read_write()
        };

        // Open a connection to the notebook's SQLite database.
        let conn = sqlite::Connection::open_with_flags(&self.nbpath, flags)
            .map_err(|e| e.to_string())?;

        // Set the connection.
        self.connection = Some(conn);

        // Return success.
        Ok(())
    }

    /// Attempts to close the notebook's SQLite database connection.
    /// If the notebook doesn't have a connection, this is a noop.
    pub fn close_db_connection(&mut self) {
        // Close the connection.
        self.connection = None;
    }

    /// Runs a function using the notebook's SQLite database connection
    /// and returns the result of that function.
    /// 
    /// If the notebook doesn't have a connection, this will attempt to
    /// open one. If it fails to open a connection (for example, if the
    /// notebook doesn't have a database path), this will return an error.
    pub fn run_with_db_connection<F, T>(&mut self, f: F) -> Result<T, String>
        where F: FnOnce(&mut sqlite::Connection) -> Result<T, String>
    {
        // Load the connection if needed.
        if self.connection.is_none() {
            self.load_db_connection()?;
        }

        // Get the connection.
        let conn = if let Some(conn) = &mut self.connection {
            conn
        } else {
            return Err("Failed to get connection".to_string());
        };

        // Run the function.
        f(conn)
    }

    /// Serialize the notebook's data to JSON.
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.data)
            .map_err(|e| e.to_string())
    }

    /// Serialize the notebook's data to JSON and write it
    /// to the notebook's file (`.nbpath`).
    pub fn save_nb_data(&mut self) -> Result<(), String> {
        // Serialize the notebook's data to JSON.
        let json = self.to_json()?;

        // Write the JSON to the notebook file.
        std::fs::write(&self.nbpath, json)
            .map_err(|e| e.to_string())?;

        // Set the save state to clean.
        self.save_state = SaveSate::Clean;

        // Return success.
        Ok(())
    }

    // List DB tables...

    // Get DB table schema...

    /// Insert a cell into the notebook cell list.
    pub fn insert_cell(&mut self, pos: usize, cell: Cell) -> Result<(), String> {
        // Check that the position is valid.
        if pos > self.data.cells.len() {
            return Err("Invalid cell index".to_string());
        }

        // Insert the cell.
        self.data.cells.insert(pos, cell);

        // Set the save state to dirty.
        if self.save_state != SaveSate::NeverSaved {
            self.save_state = SaveSate::Dirty;
        }

        // Return success.
        Ok(())
    }

    /// Update a cell in the notebook cell list.
    pub fn update_cell(&mut self, pos: usize, cell: Cell) -> Result<(), String> {
        // Check that the position is valid.
        if pos >= self.data.cells.len() {
            return Err("Invalid cell index".to_string());
        }

        // Update the cell.
        self.data.cells[pos] = cell;

        // Set the save state to dirty.
        if self.save_state != SaveSate::NeverSaved {
            self.save_state = SaveSate::Dirty;
        }

        // Return success.
        Ok(())
    }

    /// Run a query cell in the notebook.
    pub fn run_cell(&mut self, pos: usize) -> Result<(), String> {
        // Check that the position is valid.
        if pos >= self.data.cells.len() {
            return Err("Invalid cell index".to_string());
        }

        // Get the cell.
        let cell = &self.data.cells[pos];

        // Run the cell.
        match cell {
            Cell::Query(cell) => {
                // Get the query
                let query = cell.query.as_str();

                // Get the db connection
                let conn = if let Some(conn) = &mut self.connection {
                    conn
                } else {
                    return Err("Failed to get connection".to_string());
                };

                // Create a place to store the results.
                let mut results: Vec<HashMap<String, Option<String>>> = Vec::new();

                // Mark the start time
                let start = std::time::Instant::now();

                // Run the query and pull out the results.
                conn.iterate(query, |row| {
                    // Create a container for the row
                    let mut row_data: HashMap<String, Option<String>> = HashMap::new();

                    for &(col, val) in row.iter() {
                        let v = if let Some(v) = val {
                            Some(v.to_string())
                        } else {
                            None
                        };

                        row_data.insert(col.to_string(), v);
                    }

                    // Add the row to the results.
                    results.push(row_data);

                    true
                }).map_err(|e| e.to_string())?;

                // Get the query duration.
                let duration = start.elapsed();

                // Store the results in the cell.
                let mut cell = cell.clone();
                cell.results = Some(results);
                cell.query_time = Some(duration.as_millis());

                // Update the cell.
                self.update_cell(pos, Cell::Query(cell))?;

                // Mark as dirty (if needed).
                // (Note: This isn't needed, since `update_cell` does it already
                // but it's here for consistency.)
                if self.save_state != SaveSate::NeverSaved {
                    self.save_state = SaveSate::Dirty;
                }
            },
            _ => {
                return Err("Can't run non-SQL cell".to_string());
            },
        }

        // Return success.
        Ok(())
    }

    /// Swap two cells in the notebook cell list.
    pub fn swap_cells(&mut self, pos1: usize, pos2: usize) -> Result<(), String> {
        // Check that the positions are valid.
        if pos1 >= self.data.cells.len() || pos2 >= self.data.cells.len() {
            return Err("Invalid cell index".to_string());
        }

        // Swap the cells.
        self.data.cells.swap(pos1, pos2);

        // Set the save state to dirty.
        if self.save_state != SaveSate::NeverSaved {
            self.save_state = SaveSate::Dirty;
        }

        // Return success.
        Ok(())
    }

    /// Remove a cell from the notebook cell list.
    pub fn delete_cell(&mut self, pos: usize) -> Result<(), String> {
        // Check that the position is valid.
        if pos >= self.data.cells.len() {
            return Err("Invalid cell index".to_string());
        }

        // Delete the cell.
        self.data.cells.remove(pos);

        // Set the save state to dirty.
        if self.save_state != SaveSate::NeverSaved {
            self.save_state = SaveSate::Dirty;
        }

        // Return success.
        Ok(())
    }

}

#[derive(Serialize, Deserialize, Default)]
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

#[derive(Serialize, Deserialize)]
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


pub struct DbTable {
    pub name: String,
    pub columns: Vec<DbColumn>,
}

pub struct DbColumn {
    pub name: String,
    pub data_type: DbDataType,
}

pub enum DbDataType {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}


#[cfg(test)]
mod tests {
    #[test]
    fn new_client() {
        let client = super::Client::new();
        assert_eq!(client.notebooks.len(), 0);
        assert_eq!(client.nb_inc, 1);
    }

    #[test]
    fn get_new_nb_name() {
        // Create a client instance
        let mut client = super::Client::new();

        // Get a new notebook name
        let nb_name = client.get_new_nb_name();
        assert_eq!(nb_name, "notebook-1.sql.nb");
        
        // Get another new notebook name
        let nb_name = client.get_new_nb_name();
        assert_eq!(nb_name, "notebook-2.sql.nb");
    }


    #[test]
    fn new_notebook() {
        let nb = super::Notebook::new();
        assert_eq!(nb.dbpath, None);
        assert_eq!(nb.cells.len(), 0);
    }
}
