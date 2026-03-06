//! # MiniGrep
//! 
//! `minigrep` is a lightweight version of the grep command line tool.
//! This is just a Rust practice project.

/// Searches a strings contents for lines that contain a query.
/// 
/// # Example:
/// 
/// ```
/// let contents = "There is something afoot.
/// I can feel it.
/// Is someone there?";
/// let matches = minigrep::search("there", &contents);
/// 
/// assert_eq!(vec!["Is someone there?"], matches);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches a strings contents for lines that contain a query and ignores case.
/// 
/// # Example:
/// 
/// ```
/// let contents = "There is something afoot.
/// I can feel it.
/// Is someone there?";
/// let matches = minigrep::search_case_insensitive("there", &contents);
/// 
/// assert_eq!(vec!["There is something afoot.", "Is someone there?"], matches);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        );
    }
}
