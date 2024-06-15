use regex::Regex;

pub struct Preprocessor {
    max_length: usize,
    min_length: usize,
    token_pattern: Regex,
}

impl Preprocessor {
    pub fn new(max_length: usize, min_length: usize) -> Self {
        let token_pattern = Regex::new(r"\b\w+\b").unwrap();
        Preprocessor {
            max_length,
            min_length,
            token_pattern,
        }
    }

    pub fn preprocess(&self, code_snippets: &[String]) -> Vec<String> {
        code_snippets
            .iter()
            .map(|snippet| self.preprocess_snippet(snippet))
            .filter(|snippet| !snippet.is_empty())
            .collect()
    }

    fn preprocess_snippet(&self, snippet: &str) -> String {
        let snippet = self.remove_comments(snippet);
        let snippet = self.remove_extra_whitespace(&snippet);
        let snippet = self.truncate(&snippet);
        let snippet = self.pad(&snippet);
        snippet
    }

    fn remove_comments(&self, snippet: &str) -> String {
        // Remove single-line and multi-line comments from the code snippet
        // You can use regular expressions or a specific comment removal library
        // For simplicity, this example assumes that comments are removed by an external function
        remove_comments_external(snippet)
    }

    fn remove_extra_whitespace(&self, snippet: &str) -> String {
        // Remove extra whitespace, such as leading/trailing spaces and multiple consecutive spaces
        let mut result = String::new();
        let mut prev_char = ' ';
        for c in snippet.chars() {
            if c != ' ' || prev_char != ' ' {
                result.push(c);
                prev_char = c;
            }
        }
        result.trim().to_string()
    }

    fn truncate(&self, snippet: &str) -> String {
        // Truncate the code snippet if it exceeds the maximum length
        if snippet.len() > self.max_length {
            let tokens: Vec<&str> = self.token_pattern.find_iter(snippet).map(|m| m.as_str()).collect();
            let truncated_tokens = &tokens[..self.max_length];
            truncated_tokens.join(" ")
        } else {
            snippet.to_string()
        }
    }

    fn pad(&self, snippet: &str) -> String {
        // Pad the code snippet with spaces if it is shorter than the minimum length
        if snippet.len() < self.min_length {
            format!("{}{}", snippet, " ".repeat(self.min_length - snippet.len()))
        } else {
            snippet.to_string()
        }
    }
}

use tree_sitter::{Parser, Language, Node};
extern "C" { fn tree_sitter_rust() -> Language; }
extern "C" { fn tree_sitter_python() -> Language; }
extern "C" { fn tree_sitter_javascript() -> Language; }

fn remove_comments_external(snippet: &str) -> String {
    let languages = vec![
        ("rust", unsafe { tree_sitter_rust() }),
        ("python", unsafe { tree_sitter_python() }),
        ("javascript", unsafe { tree_sitter_javascript() }),
    ];

    for (lang_name, language) in languages {
        let mut parser = Parser::new();
        parser.set_language(language).unwrap();
        if let Some(tree) = parser.parse(snippet, None) {
            let root_node = tree.root_node();
            if root_node.has_error() {
                continue;
            }
            return remove_comments_from_node(snippet, root_node);
        }
    }

    snippet.to_string()
}

fn remove_comments_from_node(snippet: &str, node: Node) -> String {
    let mut result = String::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "comment" {
            continue;
        }
        let start = child.start_byte();
        let end = child.end_byte();
        result.push_str(&snippet[start..end]);
    }

    result
}
