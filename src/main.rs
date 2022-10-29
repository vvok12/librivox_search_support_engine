mod libgen_dump;
mod sql_conversion;

use libgen_dump::LibGenDump;
use sql_conversion::FictionSql;

fn main() {
    
    libgen_fiction_strategy();
    //println!("dump is {:?}", dump.dump);
}


fn libgen_fiction_strategy() {
    let dump = LibGenDump::load();
    let fiction_sql = dump.extract();
    
    let fiction_rows:Vec<String> = FictionSql(fiction_sql).postgres_rows_script().into_iter().take(3).collect();
    println!("first lines: \n{}", fiction_rows[0]);
}