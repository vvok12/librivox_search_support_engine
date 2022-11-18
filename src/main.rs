mod libgen_dump;
mod sql_conversion;

use std::sync::mpsc::{channel, Sender};
use std::thread;

use libgen_dump::LibGenDump;
use sql_conversion::FictionSql;

enum ProgramStage {
    Preparation,
    Ready
}

fn main() {
    let (sender, reciver) = channel::<ProgramStage>();
    spawn_fiction_download(sender);

    loop {
        match reciver.recv().unwrap() {
            ProgramStage::Preparation => println!("preparation started"),
            ProgramStage::Ready => { println!("ready"); break; },
        }    
    }
}

fn spawn_fiction_download(sender: Sender<ProgramStage>) {
    thread::spawn(move|| {
        sender.send(ProgramStage::Preparation).unwrap();
        
        let fiction_sql_file = LibGenDump::load().extract();
        upload(FictionSql(fiction_sql_file));
        
        sender.send(ProgramStage::Ready).unwrap()
    });
}

fn upload(_fiction_sql: FictionSql) {
    println!("UPLOADING TO DB");//todo!()
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