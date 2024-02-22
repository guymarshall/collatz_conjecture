use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use num::BigInt;
use std::path::Path;

pub fn write_to_file(file_name: &str, number: &BigInt, highest_steps: &BigInt) -> io::Result<()> {
    let mut file: File = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;

    writeln!(file, "{},{}", number, highest_steps)?;

    Ok(())
}

pub fn read_last_values(file_name: &str) -> io::Result<Vec<BigInt>> {
    let path: &Path = Path::new(file_name);
    let mut result: Vec<BigInt> = vec![
        BigInt::from(1),
        BigInt::from(0),
    ];

    if path.exists() && path.metadata()?.len() > 0 {
        let file: File = File::open(file_name)?;
        let reader: BufReader<File> = BufReader::new(file);

        for line in reader.lines() {
            let line: String = line?;
            let values: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

            if values.len() == 2 {
                result[0] = values[0].parse().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, e)
                })?;
                result[1] = values[1].parse().map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, e)
                })?;
            }
        }
    }

    Ok(result)
}