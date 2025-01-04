use diesel::connection;
use rustworkx_core::petgraph::prelude::DiGraph;
use rustworkx_core_sql_serialization::{self as lib, connect, schema::edges::payload};

fn main() {
    let dbpath = match std::env::var("DBPATH") {
        Ok(val) => val,
        Err(e) => {
            println!("couldn't interpret {e}");
            return;
        }
    };

    let connection = match connect(&dbpath) {
        Ok(val) => val,
        Err(e) => {
            println!("couldn't interpret {e}");
            return;
        }
    };
    let mut graph = DiGraph::<String, String>::new();
    let node1 = graph.add_node("aaa".to_string());
    let node2 = graph.add_node("bbb".to_string());
    graph.add_edge(node1, node2, "payload".to_string());
    connection.save(graph);
}
