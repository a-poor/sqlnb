use std::collections::HashMap;
use crate::active_notebook::ActiveNotebook;

pub struct Client {
    /// Notebooks the client is currently managing.
    /// 
    /// Maps notebook filepaths to notebook objects.
    pub notebooks: HashMap<String, ActiveNotebook>,

    pub connections: HashMap<String, Box<sqlite::Connection>>,

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
            connections: HashMap::new(),
            nb_inc: 1,
        }
    }

    /// Checks if a notebook with the given name exists
    /// in the client's notebook map.
    pub fn does_notebook_exist(&self, name: &str) -> bool {
        self.notebooks.contains_key(name)
    }

    /// Return a new notebook name, using the client's
    /// notebook counter.
    fn fmt_new_nb_name(&mut self) -> String {
        // Format the notebook name, using the counter.
        let nb_id = format!("notebook-{}.sql.nb", self.nb_inc);

        // Return the notebook name.
        nb_id
    }

    /// Return a new notebook name, using the client's
    /// notebook counter.
    pub fn get_new_nb_name(&mut self) -> String {
        // Format the first name to test
        let mut name = self.fmt_new_nb_name();

        while self.does_notebook_exist(&name) {
            // Increment the counter.
            self.nb_inc += 1;

            // Create a new name
            name = self.fmt_new_nb_name();
        }

        // Increment the counter.
        self.nb_inc += 1;

        // Return the notebook name.
        name
    }

    /// Create a new notebook.
    pub fn create_notebook(&mut self, name: Option<String>) -> String {
        // Get the notebook name.
        let nb_name = match name {
            Some(name) => name,
            None => self.get_new_nb_name(),
        };

        // Create the notebook.
        let nb = ActiveNotebook::new(nb_name.clone());

        // Add the notebook to the client's notebook map.
        self.notebooks.insert(nb_name.clone(), nb);

        // Return the notebook name.
        nb_name
    }

    fn with_conn<F, T>(&self, name: &str, f: F) -> Result<T, String>
    where
        F: FnOnce(&sqlite::Connection) -> Result<T, String>,
    {
        // Get the connection
        let conn = self.connections
            .get(name)
            .ok_or(format!("No connection named {}", name))?;


        // Call the function
        f(conn)
    }

    fn get_nb(&self, name: &str) -> Result<&ActiveNotebook, String> {
        // Get the notebook
        let nb = self.notebooks
            .get(name)
            .ok_or(format!("No notebook named {}", name))?;

        // Return the notebook
        Ok(nb)
    }

    // pub fn with_nb<F, T>(&self, name: String, f: F) -> Result<T, String>
    //     where F: FnOnce(&ActiveNotebook) -> Result<T, String>
    // {
    //     // Get the notebook.
    //     let nb = self.notebooks
    //         .get(name.as_str());

    //     match nb {
    //         Some(nb) => f(nb), // Run the function
    //         None => Err(format!("Notebook '{}' does not exist.", name)),
    //     }
    // }
}
