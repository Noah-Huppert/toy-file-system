extern crate bincode;
extern crate serde;

use std::env;
use std::process::exit;
use std::path::Path;
use std::fs::File;

use serde::{Serialize,Deserialize};

// Returns a command line interface usage string
fn usage(progname: &str) -> String {
    return format!("usage: {} DISK_FILE

Arguments:

    DISK_FILE is the file used as a disk
", progname);
}

// Current version of file system
const FS_VERSION: u8 = 0;

// Maximum number of top level inodes.
const MAX_ROOT_INODES: u8 = 128;

// Holds metadata about entire file system
#[derive(Serialize,Deserialize)]
struct FSMetadata {
    // Version of file system
    version: u8,

    // Indicates if a root inode is being used
    root_inode_usage: u128,
}

impl FSMetadata {
    fn new() -> FSMetadata {
        FSMetadata{
            version: FS_VERSION,
            root_inode_usage: 0,
        }
    }
}

fn main() {
    // Parse CLI args
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{}", usage(args[0].as_str()));
        eprintln!("DISK_FILE argument required");
        exit(1);
    }

    let arg_disk_file = args[1].as_str();

    // Open or create disk file
    let disk_file_path = Path::new(&arg_disk_file);
    let mut disk_file: File;

    let fs_metadata: FSMetadata;
    
    if !disk_file_path.exists() {
        disk_file = File::create(disk_file_path)
            .expect("failed to create disk file");

        fs_metadata = FSMetadata::new();

        bincode::serialize_into(disk_file, &fs_metadata)
            .expect("failed to write file system metadata into new disk file");
    } else {
        disk_file = File::open(disk_file_path)
            .expect("failed to open disk file");


        fs_metadata = bincode::deserialize_from(disk_file)
            .expect("failed to read file system metadata from disk file");
    }

    println!("version: {}", fs_metadata.version);
}
