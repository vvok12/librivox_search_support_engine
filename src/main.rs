mod libgen_dump;
mod sql_conversion;

use libgen_dump::LibGenDump;
use sql_conversion::FictionSql;

fn main() {
    
    libgen_fiction_strategy();
}


fn libgen_fiction_strategy() {
    let fiction_sql_file = LibGenDump::load().extract();
    upload(FictionSql(fiction_sql_file));
}

fn upload(_fiction_sql: FictionSql) {
    todo!()
}

#[test]
fn select_sql() {
    use std::fs::File;
    use std::io::prelude::*;
    

    let fiction_sql = File::open("fiction/fiction.sql").unwrap();

    let fiction = FictionSql(fiction_sql);
    let fiction_rows:Vec<String> = fiction.insert_rows_scripts().take(1).map(|r| r.unwrap()).collect();
    let _fiction_tables = fiction.tables_script();

    _ = File::create("fiction/fiction_rows.txt").unwrap().write(fiction_rows[0].as_bytes());
} 