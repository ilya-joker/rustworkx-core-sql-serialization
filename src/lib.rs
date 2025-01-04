use diesel::{
    migration::MigrationConnection, sqlite::SqliteConnection, Connection, ConnectionError,
    ConnectionResult,
};

pub mod models;
pub mod schema;

pub struct Error {
    text: String,
}

impl From<ConnectionError> for Error {
    fn from(value: ConnectionError) -> Self {
        Error {
            text: format!("{value}"),
        }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error {
            text: format!("{value}"),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

pub fn connect(database_url: &str) -> Result<(), Error> {
    let mut connection = SqliteConnection::establish(database_url)?;
    connection.setup()?;
    Ok(())
}
