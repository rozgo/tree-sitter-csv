//! This crate provides CSV, PSV, & TSV language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [language][language func] function to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! use tree_sitter::Parser;
//!
//! let code = r#"
//! "Name","Age","Salary"
//! "John Doe",30,120000
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_csv::LANGUAGE_CSV;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading CSV grammar");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//!
//! let code = r"
//! Name|Age|Salary
//! John Doe|30|120000
//! ";
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_csv::LANGUAGE_PSV;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading PSV grammar");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//!
//! let code = r"
//! Name\tAge\tSalary
//! John Doe\t30\t120000
//! ";
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_csv::LANGUAGE_TSV;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading TSV grammar");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_csv() -> *const ();
    fn tree_sitter_psv() -> *const ();
    fn tree_sitter_tsv() -> *const ();
}

/// Get the tree-sitter [Language][]
/// The tree-sitter [`LanguageFn`] for CSV grammar.
pub const LANGUAGE_CSV: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_csv) };

/// The tree-sitter [`LanguageFn`] for PSV grammar.
pub const LANGUAGE_PSV: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_psv) };

/// The tree-sitter [`LanguageFn`] for TSV grammar.
pub const LANGUAGE_TSV: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_tsv) };

/// The content of the [`grammar.json`][] file for CSV.
pub const GRAMMAR_JSON_CSV: &str = include_str!("../../csv/src/grammar.json");

/// The content of the [`grammar.json`][] file for PSV.
pub const GRAMMAR_JSON_PSV: &str = include_str!("../../psv/src/grammar.json");

/// The content of the [`grammar.json`][] file for TSV.
pub const GRAMMAR_JSON_TSV: &str = include_str!("../../tsv/src/grammar.json");

/// The content of the [`node-types.json`][] file for CSV.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_CSV: &str = include_str!("../../csv/src/node-types.json");

/// The content of the [`node-types.json`][] file for PSV.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_PSV: &str = include_str!("../../psv/src/node-types.json");

/// The content of the [`node-types.json`][] file for TSV.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES_TSV: &str = include_str!("../../tsv/src/node-types.json");

/// The syntax highlighting query for CSV.
pub const HIGHLIGHT_QUERY_CSV: &str = include_str!("../../csv/queries/highlights.scm");

/// The syntax highlighting query for PSV.
pub const HIGHLIGHT_QUERY_PSV: &str = include_str!("../../psv/queries/highlights.scm");

/// The syntax highlighting query for TSV.
pub const HIGHLIGHT_QUERY_TSV: &str = include_str!("../../tsv/queries/highlights.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE_CSV.into())
            .expect("Error loading CSV grammar");

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE_PSV.into())
            .expect("Error loading PSV grammar");

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE_TSV.into())
            .expect("Error loading TSV grammar");
    }
}
