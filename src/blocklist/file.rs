use std::fs;
use crate::dns::dto::name::Name;
use crate::cli::Args;

pub fn is_blocked(domain: &Name) -> bool {
    let blocklist_file: String = Args::get_params().file;

    let handle = fs::read_to_string(blocklist_file);

    match handle {
        Ok(blocklist) => {
            return blocklist.split("\n").any(|d| d == domain.to_string());
        },
        Err(_) => {
            log::error!("File list not available, no filtering will be possible");
            return false;
        },
    }
}