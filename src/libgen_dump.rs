use std::fs::File;
use std::io::{Write};

use unrar::Archive;

const DUMP_LINK: &str = "https://www.libgen.is/dbdumps/";
const DUMP_FICTION_RAR: &str = "fiction.rar";
const DUMP_FICTION_EXTRACT: &str = "./fiction/";
//const DUMP_FICTION_SQL: &str = "fiction.sql";

//https://www.libgen.is/dbdumps/libgen_compact.rar

pub(crate) struct LibGenDump {
    pub dump: Option<File>
}

impl LibGenDump {
    pub(crate) fn load() -> LibGenDump {
        LibGenDump {
            dump: open_file().or_else(download_file).or(None)
        }
    }

    pub(crate) fn extract(&self) -> File {
        match &self.dump {
            Some(_) => {
                Archive::new(DUMP_FICTION_RAR.to_string())
                    .extract_to(DUMP_FICTION_EXTRACT.to_string()).unwrap()
                    .process().unwrap();
            },
            None => ()
        };

        open_extracted_file()
    }
}

fn open_extracted_file() -> File {
    let mut file = String::from(DUMP_FICTION_EXTRACT);
    file.push_str(&DUMP_FICTION_RAR.replace("rar", "sql"));

    File::open(file).unwrap()
}

fn open_file() -> Option<File> {
    println!("trying to open file");

    match File::open(DUMP_FICTION_RAR) {
        Ok(file) => Some(file),
        Err(_) => None,
    }
}

fn download_file() -> Option<File> {
    println!("trying to download file");

    let link = get_fiction_download_link();
    println!("link is {}", link);


    //let get_response = reqwest::blocking::get(link); 
    let connect_timeout = Some(core::time::Duration::from_secs(30));
    let get_response = 
        reqwest::blocking::Client::builder()
            .timeout(None)
            .connect_timeout(connect_timeout)
            .build().expect("download builder failed")
            .get(link).send();
    println!("response is {:?}", get_response);

    let res = match get_response.and_then(|res| res.bytes()) {
        Ok(content) => { push_to_file(content); open_file() },
        Err(_) => None
    };
    res
}

fn get_fiction_download_link() -> String {
    let mut res = String::from(DUMP_LINK);
    res.push_str(DUMP_FICTION_RAR);
    
    res
}

fn push_to_file(content: bytes::Bytes) {
    let mut file = File::create(DUMP_FICTION_RAR).expect("pushing downloaded dump rar failed");
    file.write_all(&content).expect("file write failed");
}
