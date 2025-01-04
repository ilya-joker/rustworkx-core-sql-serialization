use rustworkx_core_sql_serialization::{self as lib, connect};

fn main() {
    let dbpath = match std::env::var("DBPATH") {
        Ok(val) => val,
        Err(e) => {
            println!("couldn't interpret {e}");
            return;
        }
    };
    match connect(&dbpath) {
        Ok(_val) => println!("Connected"),
        Err(e) => {
            println!("couldn't interpret {e}");
            return;
        }
    }
}
