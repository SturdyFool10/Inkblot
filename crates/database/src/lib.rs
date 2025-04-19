use rusqlite::Connection;
use std::error::Error;
use std::path::Path;

pub struct User {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub permissions: u16,
}

pub struct Db {
    conn: Connection,
}

impl Db {
    /// Creates a new `Db` instance by opening a database connection from the specified file path.
    ///
    /// # Arguments
    /// * `path` - A string slice that specifies the path to the database file.
    ///
    /// # Returns
    /// * `Ok(Self)` - If the database connection is successfully established and initialized.
    /// * `Err(Box<dyn Error>)` - If an error occurs during the process, such as an empty path,
    ///   failure to create the database file, or failure to execute the initialization script.
    ///
    /// # Errors
    /// This function will return an error if:
    /// * The provided `path` is empty.
    /// * The database file cannot be created if it does not exist.
    /// * The database connection cannot be opened.
    /// * The initialization script cannot be executed.
    ///
    /// # Notes
    /// * The initialization script is included at compile time using the `include_str!` macro.
    /// * If the database file does not exist, it will be created automatically.
    ///
    /// # Example
    /// ```
    /// let db = Db::from_path("database.db");
    /// match db {
    ///     Ok(db_instance) => println!("Database initialized successfully."),
    ///     Err(e) => eprintln!("Failed to initialize database: {}", e),
    /// }
    /// ```
    pub fn from_path(path: &str) -> Result<Self, Box<dyn Error>> {
        if path.is_empty() {
            return Err("Path cannot be empty".into()); //ensure path is not ""
        }

        // Create the database file if it doesn't exist
        if !Path::exists(Path::new(path)) {
            std::fs::File::create(path)?;
        }

        let init_script = include_str!("/sql/init_db.sql"); //includes the script at compile time, no error handling required
        let conn = Connection::open(path)?;
        conn.execute(init_script, [])?;

        Ok(Self { conn })
    }
}
