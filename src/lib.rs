use diesel::{
    migration::MigrationConnection, sqlite::SqliteConnection, Connection, ConnectionError,
    RunQueryDsl,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use models::Vertex;
use rustworkx_core::petgraph::graph::DiGraph;
use schema::vertices;

pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Error {
    text: String,
}

pub struct GraphConnection {
    connection: SqliteConnection,
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

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(value: Box<dyn std::error::Error + Send + Sync>) -> Self {
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

impl GraphConnection {
    pub fn save(&mut self, graph: DiGraph<Vertex, String>) -> Result<(), Error> {
        for n in graph.node_indices() {
            diesel::insert_into(vertices::table)
                .values(&graph[n])
                .execute(&mut self.connection)?;
        }
        Ok(())
    }
}

pub fn connect(database_url: &str) -> Result<GraphConnection, Error> {
    let mut connection = SqliteConnection::establish(database_url)?;
    connection.setup()?;
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(GraphConnection {
        connection: connection,
    })
}
