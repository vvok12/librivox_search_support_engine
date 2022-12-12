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


#[cfg(test)]
mod fiction_test {
    use crate::sql_conversion::FictionSql;
    use std::fs::File;
    use std::io::{prelude::*, self};
    use postgres::{Client, NoTls, GenericClient};

    const POSTGRES_CONNECTION_STRING: &str = "host=localhost user=postgres password=admin dbname=libgen";

    fn get_fiction_txt() -> io::Result<File> {
        File::open("fiction/fiction.sql")
    }

    fn get_fiction_rows<'a>(count: usize) -> io::Result<(FictionSql, Vec<String>)> {
        let fiction = FictionSql(get_fiction_txt()?);
        let rows = fiction.get_rows_insert_scripts().take(count).map(|r| r.unwrap()).collect();
        Ok((fiction, rows))
    }

    fn reshape_postgres_row(row:String) -> Vec<String> {
        const VALUES_KEYWORD:&str = "VALUES";

        let til_values = row.find(VALUES_KEYWORD).unwrap() + VALUES_KEYWORD.len() + 1;
        let pattern = &row[0..til_values];
        let tail = &row[til_values..];

        let pattern = pattern.replace('`', "")
            .replace("GooglebookID", "\"googlebookID\"")
            .replace("ASIN","\"ASIN\"")
            .replace("TimeAdded","\"TimeAdded\"")
            .replace("TimeLastModified", "\"TimeLastModified\"");

        tail.replace("\\'", "`").split("),(").into_iter().map(
            |s| {
                let mut x = pattern.clone();
                if s.chars().nth(0) != Some('(') { 
                    x.push('(')
                }
                x.push_str(s);
                if s.chars().last() != Some(')') {
                    x.push(')')
                }
                x.push_str(";\r\n");

                x
            }
        ).collect()
    }

    #[test]
    fn get_fiction_txt_test() {
        get_fiction_txt().expect("could not open fiction.txt");
    }

    #[test]
    fn write_fiction_rows_file() {
        let (_fiction, mut rows) = get_fiction_rows(1).unwrap();
        let rows = reshape_postgres_row(rows.pop().unwrap());
        let lines = rows.iter().map(|r| r.as_bytes()).collect::<Vec<&[u8]>>();
        let mut file = File::create("fiction/fiction_rows.txt").expect("open file to write");
        for line in lines {
            file.write(line).unwrap();
        }
    }

    #[test] 
    fn connect_to_postgres_client() {
        Client::connect(POSTGRES_CONNECTION_STRING, NoTls).unwrap();
    }

    #[test]
    fn load_fiction_rows_to_db() {
        let (_, mut rows) = get_fiction_rows(1).unwrap();
        let rows = reshape_postgres_row(rows.pop().unwrap());

        for row in &rows[0..10] {
            println!("rows: {:?} \n", row);
        }

        let mut client = Client::connect(POSTGRES_CONNECTION_STRING, NoTls).unwrap();
        //client.execute("delete from fiction;", &[]).unwrap();
        client.execute(&rows[2] as &str, &[]).unwrap();
    }
}
 

