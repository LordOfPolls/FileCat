use std::collections::HashMap;
use std::fs::{DirEntry, File};
use std::io::{self};

use csv::{ReaderBuilder, WriterBuilder};

pub fn concat(files: Vec<DirEntry>) -> Result<(), std::io::Error> {
    let mut writer = WriterBuilder::new()
        .quote_style(csv::QuoteStyle::Necessary)
        .from_writer(io::stdout());

    let mut header_map = HashMap::new();
    let mut header_written = false;

    for file in files {
        let file = File::open(file.path())?;
        let mut reader = ReaderBuilder::new()
            .flexible(true)
            .from_reader(io::BufReader::new(file));

        let headers = reader.headers()?;
        for (idx, header) in headers.iter().enumerate() {
            header_map.entry(header.to_string()).or_insert(idx);
        }

        if !header_written {
            writer.write_record(header_map.keys())?;
            header_written = true;
        }

        for result in reader.records() {
            let record = result?;
            let output_record: Vec<_> = header_map
                .iter()
                .map(|(_header, &idx)| record.get(idx).unwrap_or(""))
                .collect();

            writer.write_record(&output_record)?;
        }
    }

    writer.flush()?;
    Ok(())
}
