use std::fs;

use crate::tools::search_in_file;

pub fn search(
    needle: &str,
    haystack: &str,
    ignore_case: bool,
    line_numbers: bool,
    count_matches: bool,
    invert_match: bool,
) {
    match fs::read_to_string(haystack) {
        Ok(contents) => {
            search_in_file::search_in_file(
                needle,
                contents.as_str(),
                ignore_case,
                line_numbers,
                count_matches,
                invert_match,
            );
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
