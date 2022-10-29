use std::fs::File;
use std::io::{BufReader, BufRead};

pub(crate) struct FictionSql(pub(crate) File);

impl FictionSql {
    fn postgres_tables_script(self) -> Vec<String> {
        vec![]
    }

    pub(crate) fn postgres_rows_script(self) -> Vec<String> {
        BufReader::new(self.0)
            .lines()
            .filter(|line_read_result| line_read_result.is_ok() &&  { 
                let line: &String = line_read_result.as_ref().unwrap();
                line.trim_start().to_lowercase().starts_with("insert")
            })
            .map(|line_result| line_result.unwrap())
            .collect()
    }
}