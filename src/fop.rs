use std::fs;
use std::io;
use std::path::PathBuf;
use itertools::Itertools;

pub fn validate_file(filename: &PathBuf) -> Result<&PathBuf, io::Error> {
    let stat = fs::metadata(filename)?;
    let file_type = stat.file_type();

    match file_type.is_file() {
        true => return Ok(filename),
        false => return Err(io::Error::new(io::ErrorKind::Other, "Not a regular file")),
    };
}

pub fn validate_files(src_file: &PathBuf, dst_file: &PathBuf) -> Result<bool, io::Error> {
    let src_stat = fs::metadata(src_file)?;
    let s_file_type = src_stat.file_type();

    let dst_stat = fs::metadata(dst_file)?;
    let d_file_type = dst_stat.file_type();

    match (s_file_type.is_file(), d_file_type.is_file()) {
        (true,true) => return Ok(true),
        _ => return Err(io::Error::new(io::ErrorKind::Other, "Not a regular file")),
    };
}

pub fn dedupe_files(files_list: Vec<PathBuf>, dry_run: bool) {
    if dry_run {
            println!("dry run mode");
    }
    for file in &files_list {
        // TODO: Print error message neatly
        match validate_file(file) {
            Ok(file) => println!("{:#?} is a regular file.", file),
            Err(error) => println!("{:#?} Error: {}", file, error),
        };
    }
    let comb = files_list.iter().combinations(2).collect::<Vec<_>>();
    for f in &comb{
        match validate_files(f[0], f[1]) {
            Ok(_) => println!(" {:#?} {:#?} are valid files.", f[0], f[1]),
            Err(error) => println!("Error: {}",  error),
        };
    }
}

pub fn dedupe_dir(dir_path: Vec<PathBuf>, dry_run: bool, recurse: bool) -> io::Result<()> {
    let mut entries = Vec::new();
    if dry_run {
            println!("dry run mode");
    }
    if recurse {
            println!("recurse mode");
    }
    for dir in &dir_path {
        entries = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
    }
    for f in &entries {
        println!("{:#?}", f);
    }
    dedupe_files(entries, dry_run);
    Ok(())
}
