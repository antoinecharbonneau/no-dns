use crate::cli::Args;
use crate::dns::dto::name::Name;
use std::fs::File;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::path::Path;
use lazy_static::lazy_static;
use fasthash::city;


lazy_static!{
    static ref BLOCKLIST: HashSet<String, city::Hash64> = init();
}

fn init() -> HashSet<String, city::Hash64> {
    let blocklist_file: String = Args::get_params().file;

    let mut hs: HashSet<String, city::Hash64> = HashSet::with_capacity_and_hasher(16, city::Hash64);
    
    match read_lines(blocklist_file) {
        Ok(lines) => {
            for line in lines {
                if let Ok(content) = line {
                    if content.len() > 0 {
                        hs.insert(content);
                    }
                }
            }
        },
        Err(_) => log::error!("File list not available, no filtering will be possible"),
    }

    hs
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn is_blocked(domain: &Name) -> bool {
    BLOCKLIST.contains(&domain.get_string())
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
