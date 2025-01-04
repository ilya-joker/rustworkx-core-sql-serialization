use diesel::{sqlite::SqliteConnection, Connection, ConnectionResult};

pub fn connect(database_url: &str) -> ConnectionResult<()> {
    SqliteConnection::establish(database_url).map(|_| ())
}
