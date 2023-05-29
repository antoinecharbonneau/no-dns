use crate::cli::Args;
use crate::dns::dto::name::Name;
use std::fs;

pub fn is_blocked(domain: &Name) -> bool {
    let blocklist_file: String = Args::get_params().file;

    let handle = fs::read_to_string(blocklist_file);

    match handle {
        Ok(blocklist) => match_blocked(&blocklist, &domain.to_string()),
        Err(_) => {
            log::error!("File list not available, no filtering will be possible");
            return false;
        }
    }
}

fn match_blocked(blocklist: &String, domain: &str) -> bool {
    let domain_lower = domain.to_lowercase();
    return blocklist.split("\n").any(|d: &str| {
        let d = &d.trim().to_lowercase();
        // exact match
        return d == &domain_lower
        // patern match (*.example.com)
        || d.starts_with("*.") && domain_lower.ends_with(&d[2..d.len()]);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_blocked() {
        let blocklist = String::from(
            "
        youtube.com
        google.com
        *.test.ca
        www.eXample.com
        ",
        );

        assert!(match_blocked(&blocklist, "youtube.com"));
        assert!(!match_blocked(&blocklist, "www.google.com"));
        assert!(match_blocked(&blocklist, "test2.test.ca"));
        assert!(match_blocked(&blocklist, "test.ca"));
        assert!(!match_blocked(&blocklist, "test.ca.google.com"));
        assert!(match_blocked(&blocklist, "www.example.com"));
    }
}
