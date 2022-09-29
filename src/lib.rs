use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use serde::{Serialize, Deserialize};


pub struct Client {
    /// Notebooks the client is currently managing.
    /// 
    /// Maps notebook filepaths to notebook objects.
    pub notebooks: Arc<Mutex<HashMap<String, Box<ActiveNotebook>>>>,

    /// A counter for creating new notebook IDs.
    /// 
    /// For example, a new notebook name may be of
    /// the form "notebook-{{ nb_inc }}.sql.nb"
    pub nb_inc: Arc<Mutex<usize>>,
}

impl Client {
    /// Create a new instance of Client.
    pub fn new() -> Client {
        Client {
            notebooks: Arc::new(Mutex::new(HashMap::new())),
            nb_inc: Arc::new(Mutex::new(1)),
        }
    }

    pub fn does_notebook_exist(&self, name: &str) -> Result<bool, String> {
        // Get the notebooks.
        let nbs = if let Ok(nbs) = self.notebooks.lock() {
            nbs
        } else {
            return Err("Couldn't obtain lock for notebook data".to_string());
        };

        // Return true if the notebook exists.
        Ok(nbs.contains_key(name))
    }

    /// Return a new notebook name, using the client's
    /// notebook counter.
    pub fn get_new_nb_name(&self) -> Result<String, String> {
        // Get the client's notebook counter.
        let mut nb_inc = if let Ok(nb_inc) = self.nb_inc.lock() {
            nb_inc
        } else {
            return Err("Failed to lock nb_inc".to_string());
        };

        // Format the notebook name, using the counter.
        let nb_id = format!("notebook-{}.sql.nb", *nb_inc);

        // Increment the counter.
        *nb_inc += 1;

        // Return the notebook name.
        Ok(nb_id)
    }
}

/// A notebook's save-state, on disk.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownCell {
    /// The cell's markdown contents
    pub contents: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryCell {
    /// The cell's query contents
    pub query: String,

    /// The cell's query results
    pub results: Option<HashMap<String, Vec<Value>>>,
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
        assert_eq!(client.notebooks.lock().unwrap().len(), 0);
        assert_eq!(*client.nb_inc.lock().unwrap(), 1);
    }

    #[test]
    fn get_new_nb_name() {
        // Create a client instance
        let client = super::Client::new();

        // Get a new notebook name
        let nb_name = client.get_new_nb_name().unwrap();
        assert_eq!(nb_name, "notebook-1.sql.nb");
        
        // Get another new notebook name
        let nb_name = client.get_new_nb_name().unwrap();
        assert_eq!(nb_name, "notebook-2.sql.nb");
    }


    #[test]
    fn new_notebook() {
        let nb = super::Notebook::new();
        assert_eq!(nb.dbpath, None);
        assert_eq!(nb.cells.len(), 0);
    }
}
