#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Bytes, Read};
use std::thread::JoinHandle;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();
    let files = matches.values_of_lossy("files").unwrap();
    let lines = matches
                        .value_of("lines")
                        .map(parse_positive_int)
                        .transpose()
                        .map_err(|e| format!("illegal line count -- {} ", e))?;
                        
    let bytes = matches
            .value_of("bytes")
            .map(parse_positive_int)
            .transpose()
            .map_err(|e| format!("illegal byte count -- {} ", e))?;

    Ok(Config {
        files: files,
        lines: lines.unwrap(),
        bytes: bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                                if num_files > 1 {println!("{}==> {} <==",
                                if file_num > 0 {"\n"} else {""},
                                filename);}

                                if config.bytes.is_some() { 
                                    //read_bytes( file, config.bytes.expect("Bytes value not found"))
                                    read_bytes_v2( file, config.bytes.expect("Bytes value not found"))
                                        }
                                else {
                                    //read_lines(file, config.lines)
                                    read_lines_v2(file, config.lines)
                                }
                                },
        }
    }

    Ok(())
}

fn read_bytes_v2(mut file: Box<dyn BufRead>, num_bytes: usize) {
    let mut handle = file.take(num_bytes as u64);
    let mut buffer = vec![0; num_bytes];
    let bytes_read = handle.read(&mut buffer).expect("Failed to read the file");
    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
}

fn read_lines(file: Box<dyn BufRead>, lines: usize) {
    for (count, curr_line )in file.lines().enumerate() {
        println!("{}", curr_line.unwrap());
        if count + 1 == lines { break;}
    }
    println!("");
}

fn read_lines_v2(mut file: Box<dyn BufRead>, lines: usize) {
    let mut line = String::new();

    for _ in 0..lines {
        let num_bytes = file.read_line(&mut line).expect("Failed to read next line");
        if num_bytes == 0 {break;}
        print!("{}", line);
        line.clear();
    }
}

fn read_bytes(mut file: Box<dyn BufRead>, bytes: usize) {
    let mut buffer = Vec::new();
    let mut s : String = "".to_string();
    // read the whole file
    file.read_to_end(&mut buffer).expect("Failed to read bytes");

    let mut i = 0;
    while i < bytes {
        let d = String::from_utf8_lossy(buffer.as_slice());
        s.push_str(&d);
        i = i + 1;
    }
    
    println!("{}", String::from_utf8(buffer).unwrap());

}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    //3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    //Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());

    //A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}



// --------------------------------------------------
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}








