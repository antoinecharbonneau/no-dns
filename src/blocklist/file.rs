use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::cli::Args;
use crate::dns::dto::name::Name;

pub fn get_elements() -> Vec<Name> {
    let blocklist_file: String = Args::get_params().file;

    let mut list: Vec<Name> = Vec::new();
    
    match read_lines(blocklist_file) {
        Ok(lines) => {
            for line in lines {
                if let Ok(content) = line {
                    if content.len() > 0 {
                        list.push(Name::from(content.as_str()));
                    }
                }
            }
        },
        Err(_) => log::error!("File list not available, no filtering will be possible"),
    }

    list
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    // use super::*;
    //
    // #[test]
    // fn test_match_blocked() {
    //     let blocklist = String::from(
    //         "
    //     youtube.com
    //     google.com
    //     *.test.ca
    //     www.eXample.com
    //     ",
    //     );
    //
    //     assert!(match_blocked(&blocklist, "youtube.com"));
    //     assert!(!match_blocked(&blocklist, "www.google.com"));
    //     assert!(match_blocked(&blocklist, "test2.test.ca"));
    //     assert!(match_blocked(&blocklist, "test.ca"));
    //     assert!(!match_blocked(&blocklist, "test.ca.google.com"));
    //     assert!(match_blocked(&blocklist, "www.example.com"));
    // }
}
