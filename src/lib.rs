use std::collections::HashMap;
use serde::{Serialize, Deserialize};

mod client;
mod notebook;
mod active_notebook;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn new_client() {
//         let client = super::Client::new();
//         assert_eq!(client.notebooks.len(), 0);
//         assert_eq!(client.nb_inc, 1);
//     }

//     #[test]
//     fn get_new_nb_name() {
//         // Create a client instance
//         let mut client = super::Client::new();

//         // Get a new notebook name
//         let nb_name = client.get_new_nb_name();
//         assert_eq!(nb_name, "notebook-1.sql.nb");
        
//         // Get another new notebook name
//         let nb_name = client.get_new_nb_name();
//         assert_eq!(nb_name, "notebook-2.sql.nb");
//     }

//     #[test]
//     fn get_new_nb_name_with_skips() {
//         // Create a client instance
//         let mut client = super::Client::new();

//         // Create new notebooks (to increment the counter)
//         client.create_notebook(None);
//         client.create_notebook(None);

//         // Reset the notebook counter
//         client.nb_inc = 1;
        
//         // Get another new notebook name
//         let nb_name = client.get_new_nb_name();
//         assert_eq!(nb_name, "notebook-3.sql.nb");
//     }


//     #[test]
//     fn new_notebook() {
//         let nb = super::Notebook::new();
//         assert_eq!(nb.dbpath, None);
//         assert_eq!(nb.cells.len(), 0);
//     }

//     #[test]
//     fn client_create_nb() {
//         // Create a client instance
//         let mut client = super::Client::new();

//         // Create new notebook
//         let nb_name = client.create_notebook(None);

//         let nb = client.get_nb(&nb_name).unwrap();

//         // Check that the notebook is in the client's list
//         assert_eq!(client.notebooks.len(), 1);
//         assert!(client.does_notebook_exist(nb_name.as_str()));

//         // Check that the notebook is empty
//         assert_eq!(nb.data.cells.len(), 0);

//         // Check that the notebook doesn't have a database
//         assert_eq!(nb.data.dbpath, None);

//         // Check that the notebook is unsaved
//         assert_eq!(nb.save_state, super::SaveSate::NeverSaved);

//         // Set the database path
//         nb.set_db_path(":memory:".to_string());

//         // // Get the notebook
//         // let res = client.with_nb(nb_name, |nb| {
//         //     // Check that the notebook is in the client's list
//         //     assert_eq!(client.notebooks.len(), 1);
//         //     assert!(client.does_notebook_exist(nb_name.as_str()));

//         //     // Check that the notebook is empty
//         //     assert_eq!(nb.data.cells.len(), 0);

//         //     // Check that the notebook doesn't have a database
//         //     assert_eq!(nb.data.dbpath, None);

//         //     // Check that the notebook is unsaved
//         //     assert_eq!(nb.save_state, super::SaveSate::NeverSaved);

//         //     // Set the database path
//         //     nb.set_db_path(":memory:".to_string());

//         //     Ok(())
//         // });
//     }
// }
