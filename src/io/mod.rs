use std::io::{BufRead, BufWriter, Read, Write};
use std::fs::File;
use std::path::Path;
use error::Result;

pub fn read<R>(mut reader: R) -> Result<String>
where
    R: BufRead,
{
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn write_file<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
    let f = File::create(path)?;
    let mut b = BufWriter::new(f);
    b.write_all(contents.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use io::*;
    use mktemp::Temp;

    #[test]
    fn read_file_ok() {
        let expt = "hello";
        let file = "read_test.txt";

        let tmp = Temp::new_dir().unwrap();
        let mut tmp_file = tmp.to_path_buf();
        tmp_file.push(file);

        write_file(&tmp_file, expt).unwrap();

        let actl = read_file(&tmp_file);
        assert!(actl.is_ok());
        assert_eq!(expt, &actl.unwrap());
    }

    #[test]
    fn read_file_not_found() {
        let file = "not_found.txt";
        let tmp = Temp::new_dir().unwrap();
        let mut tmp_file = tmp.to_path_buf();
        tmp_file.push(file);
        let actl = read_file(&tmp_file);
        assert!(actl.is_err());
    }

    #[test]
    fn write_file_ok() {
        let file = "write_test.txt";
        let tmp = Temp::new_dir().unwrap();
        let mut tmp_file = tmp.to_path_buf();
        tmp_file.push(file);
        let actl = write_file(&tmp_file, "write");

        assert!(actl.is_ok());
    }
}
