fn main() {
    match std::env::var("DBPATH") {
        Ok(val) => println!("{val:?}"),
        Err(e) => println!("couldn't interpret {e}"),
    }
}
