use crate::tools::search_in_file;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::result::Result;
pub fn search(
    needle: &str,
    haystack: &str,
    ignore_case: bool,
    line_numbers: bool,
    count_matches: bool,
    invert_match: bool,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(Path::new(haystack))?;
    let reader = BufReader::new(file);
    search_in_file::search_in_file(
        needle,
        reader,
        ignore_case,
        line_numbers,
        count_matches,
        invert_match,
    )?;

    Ok(())
}
