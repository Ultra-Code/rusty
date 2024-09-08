//! This crate is for implementing cli args parsing and
//! the grepping functionality
pub mod cli;

use std::env;

pub enum Case {
    Sensitive,
    Insensitive,
}

/// Search for the query in a files content respecting the case sensitivity.
///
/// # Examples
///
/// ```
/// use book::mgrep::Case;
/// use book::mgrep::search;
///
/// let query = "5";
/// let content = "this is %\n5 is the number";
/// let case = Case::Sensitive;
/// let answer = search(query,content,case);
///
/// assert_eq!(vec!["5 is the number"], answer);
/// ```
#[allow(clippy::needless_pass_by_value)]
#[must_use]
pub fn search<'a>(query: &str, content: &'a str, case: Case) -> Vec<&'a str> {
    match case {
        Case::Insensitive => {
            let query = query.to_lowercase();

            content
                .lines()
                .filter(|line| line.to_lowercase().contains(&query))
                .collect::<Vec<&str>>()
        }
        Case::Sensitive => content
            .lines()
            .filter(|line| line.contains(query))
            .collect::<Vec<&str>>(),
    }
}

#[must_use]
pub fn case() -> Case {
    env::var("IGNORE_CASE")
        .map_or_else(|_| Case::Sensitive, |_| Case::Insensitive)
}
