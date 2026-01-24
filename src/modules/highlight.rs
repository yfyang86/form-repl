// Syntax highlighting for FORM language
use regex::Regex;
use std::sync::LazyLock;

use super::theme::Theme;

/// FORM language token types for syntax highlighting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Keyword,
    Declaration,
    Function,
    Preprocessor,
    Number,
    Operator,
    Comment,
    String,
    Identifier,
    Punctuation,
    Whitespace,
}

/// A token with its type and text content
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub text: String,
}

/// Keywords that should be highlighted
const KEYWORDS: &[&str] = &[
    "if", "else", "elseif", "endif", "while", "endwhile", "repeat", "endrepeat",
    "do", "enddo", "goto", "label", "exit", "break", "continue", "return",
    "procedure", "endprocedure", "call", "argument", "endargument",
    "switch", "case", "default", "endswitch", "inside", "endinside",
    "term", "endterm", "sort", "endsort", "multiply", "also", "once", "only",
    "multi", "all", "first", "last", "disorder", "antisymmetrize", "symmetrize",
    "cyclesymmetrize", "rcyclesymmetrize", "identify", "idnew", "idold",
    "chainout", "chainin", "splitarg", "splitfirstarg", "splitlastarg",
    "factarg", "normalize", "makeinteger", "torat", "topolynomial",
    "frompolynomial", "argtoextrasymbol", "dropcoefficient", "dropextrasymbols",
    "polyratfun", "ratfun", "keep", "drop", "hide", "unhide", "skip", "nskip",
    "moduleoption", "on", "off", "format", "write", "redefine", "renumber",
    "contract", "trace4", "tracen", "chisholm", "unittrace", "delete", "discard",
    "print", "nprint", "collect", "bracket", "antibracket", "putinside",
    "polyfun", "sum", "id", "fill", "fillexpression", "table", "ctable",
    "tablebase", "testuse", "apply", "transform", "replace", "replaceloop",
    "totensor", "tovector", "fromtensor", "metric", "dimension", "load", "save",
    "copyspecs", "setexitflag", "nwrite", "threadbucketsize", "processbucketsize",
];

/// Declaration keywords
const DECLARATIONS: &[&str] = &[
    "symbol", "symbols", "index", "indices", "vector", "vectors",
    "tensor", "tensors", "ntensor", "ntensors", "function", "functions",
    "cfunction", "cfunctions", "ctensor", "ctensors", "nfunction", "nfunctions",
    "ncfunction", "ncfunctions", "table", "tables", "ctable", "ctables",
    "set", "local", "global", "auto", "autodeclare", "dimension",
    "fixindex", "unfixindex", "extrasymbol", "extrasymbol", "commuting",
    "noncommuting",
];

/// Built-in functions (without the trailing parenthesis check for simplicity)
const FUNCTIONS: &[&str] = &[
    "abs", "sign", "min", "max", "mod", "div", "gcd", "fac", "binom",
    "bernoulli", "sqrt", "sin", "cos", "tan", "asin", "acos", "atan",
    "atan2", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh", "exp",
    "ln", "log", "log10", "li2", "li3", "nielsen", "hpl", "mzv", "zeta",
    "gamma", "polygamma", "psi", "digamma", "theta", "delta_", "d_", "e_",
    "i_", "f_", "g_", "gi_", "dd_", "conjg_", "deno", "farg", "nargs",
    "firstarg", "lastarg", "numterms", "termsin", "maxpow", "minpow",
    "exponent", "coeff", "content", "integer_", "symbol_", "index_",
    "vector_", "fixed_", "match", "count", "occurs", "multipleof", "prime",
    "random_", "tbl_", "term_", "expression_", "dummyindices", "extrasymbol_",
    "getdummies", "nterms", "sump_", "sum_", "prod_", "inv_", "root_",
    "replace_", "setfun", "putfirst", "addargs", "mulargs", "permute",
    "reverse", "delta", "epsilon", "distrib_", "sig_", "factorin_", "gcd_",
    "div_", "rem_", "inverse_", "makerational", "rat", "num_", "den_",
    "derive", "accum", "pcount_", "firstbracket_", "table_", "defined_",
    "termsinbracket_", "maxpower_", "minpower_", "ranperm_", "exists_",
    "pattern_", "setspec_", "exec_", "partitions_", "compargs_",
    "commutearg_", "sortarg_", "dedup_",
];

/// Compiled regex patterns for FORM syntax (without lookahead)
struct FormPatterns {
    preprocessor: Regex,
    number: Regex,
    operator: Regex,
    string: Regex,
    identifier: Regex,
}

// Lazily compiled regex patterns
static PATTERNS: LazyLock<FormPatterns> = LazyLock::new(|| FormPatterns {
    preprocessor: Regex::new(r"^(#[a-zA-Z]+|\.end|\.sort|\.store|\.global|\.clear)").unwrap(),
    number: Regex::new(r"^-?[0-9]+\.?[0-9]*([eE][+-]?[0-9]+)?").unwrap(),
    operator: Regex::new(r"^(==|!=|<=|>=|<>|<|>|&&|\|\||[+\-*/^?=,;:])").unwrap(),
    string: Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(),
    identifier: Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap(),
});

/// Check if an identifier is a keyword (case-insensitive)
fn is_keyword(word: &str) -> bool {
    let lower = word.to_lowercase();
    KEYWORDS.contains(&lower.as_str())
}

/// Check if an identifier is a declaration keyword (case-insensitive)
fn is_declaration(word: &str) -> bool {
    let lower = word.to_lowercase();
    DECLARATIONS.contains(&lower.as_str())
}

/// Check if an identifier is a built-in function (case-insensitive)
fn is_function(word: &str) -> bool {
    let lower = word.to_lowercase();
    FUNCTIONS.contains(&lower.as_str())
}

/// Tokenize a line of FORM code
pub fn tokenize(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut remaining = line;
    
    // Check for comment (FORM comments start with * at the beginning of a line)
    let trimmed = remaining.trim_start();
    if trimmed.starts_with('*') {
        tokens.push(Token {
            token_type: TokenType::Comment,
            text: line.to_string(),
        });
        return tokens;
    }
    
    while !remaining.is_empty() {
        // Skip whitespace but preserve it
        if remaining.starts_with(|c: char| c.is_whitespace()) {
            let ws_len = remaining
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();
            let ws: String = remaining.chars().take(ws_len).collect();
            let byte_len: usize = ws.len();
            tokens.push(Token {
                token_type: TokenType::Whitespace,
                text: ws,
            });
            remaining = &remaining[byte_len..];
            continue;
        }
        
        // Check for string literal
        if let Some(m) = PATTERNS.string.find(remaining) {
            tokens.push(Token {
                token_type: TokenType::String,
                text: m.as_str().to_string(),
            });
            remaining = &remaining[m.end()..];
            continue;
        }
        
        // Check for preprocessor directives
        if let Some(m) = PATTERNS.preprocessor.find(remaining) {
            tokens.push(Token {
                token_type: TokenType::Preprocessor,
                text: m.as_str().to_string(),
            });
            remaining = &remaining[m.end()..];
            continue;
        }
        
        // Check for identifiers (then classify as keyword/declaration/function/identifier)
        if let Some(m) = PATTERNS.identifier.find(remaining) {
            let word = m.as_str();
            let token_type = if is_declaration(word) {
                TokenType::Declaration
            } else if is_keyword(word) {
                TokenType::Keyword
            } else if is_function(word) {
                // Check if followed by '(' to confirm it's a function call
                let after = &remaining[m.end()..];
                if after.trim_start().starts_with('(') {
                    TokenType::Function
                } else {
                    TokenType::Identifier
                }
            } else {
                TokenType::Identifier
            };
            
            tokens.push(Token {
                token_type,
                text: word.to_string(),
            });
            remaining = &remaining[m.end()..];
            continue;
        }
        
        // Check for numbers
        if let Some(m) = PATTERNS.number.find(remaining) {
            tokens.push(Token {
                token_type: TokenType::Number,
                text: m.as_str().to_string(),
            });
            remaining = &remaining[m.end()..];
            continue;
        }
        
        // Check for operators
        if let Some(m) = PATTERNS.operator.find(remaining) {
            tokens.push(Token {
                token_type: TokenType::Operator,
                text: m.as_str().to_string(),
            });
            remaining = &remaining[m.end()..];
            continue;
        }
        
        // Punctuation and other characters
        if let Some(c) = remaining.chars().next() {
            tokens.push(Token {
                token_type: TokenType::Punctuation,
                text: c.to_string(),
            });
            remaining = &remaining[c.len_utf8()..];
        }
    }
    
    tokens
}

/// Highlight a single line of FORM code
pub fn highlight_line(line: &str, theme: &Theme) -> String {
    let tokens = tokenize(line);
    let reset = "\x1b[0m";
    
    tokens
        .into_iter()
        .map(|token| {
            let color = match token.token_type {
                TokenType::Keyword => &theme.keyword,
                TokenType::Declaration => &theme.declaration,
                TokenType::Function => &theme.function,
                TokenType::Preprocessor => &theme.preprocessor,
                TokenType::Number => &theme.number,
                TokenType::Operator => &theme.operator,
                TokenType::Comment => &theme.comment,
                TokenType::String => &theme.string,
                TokenType::Identifier => &theme.identifier,
                TokenType::Punctuation | TokenType::Whitespace => "",
            };
            
            if color.is_empty() {
                token.text
            } else {
                format!("{}{}{}", color, token.text, reset)
            }
        })
        .collect()
}

/// Highlight multiple lines of FORM code
pub fn highlight_code(code: &str, theme: &Theme) -> String {
    code.lines()
        .map(|line| highlight_line(line, theme))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Highlight FORM output (results from computation)
pub fn highlight_output(output: &str, theme: &Theme) -> String {
    let reset = "\x1b[0m";
    let lines: Vec<&str> = output.lines().collect();
    let mut result = Vec::new();
    
    for line in lines {
        let trimmed = line.trim();
        
        // Expression assignment lines (e.g., "   E =")
        if trimmed.ends_with(" =") || trimmed == "=" {
            result.push(format!("{}{}{}", theme.output_label, line, reset));
        }
        // Timing lines
        else if trimmed.contains("sec out of") || trimmed.starts_with("Time =") {
            result.push(format!("{}{}{}", theme.timing, line, reset));
        }
        // Error/warning lines
        else if trimmed.starts_with("Error") || trimmed.starts_with("Warning") {
            result.push(format!("{}{}{}", theme.error, line, reset));
        }
        // Expression content - highlight the math
        else if !trimmed.is_empty() {
            result.push(highlight_line(line, theme));
        } else {
            result.push(line.to_string());
        }
    }
    
    result.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tokenize_declaration() {
        let tokens = tokenize("Symbol x,y;");
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Declaration && t.text == "Symbol"));
    }
    
    #[test]
    fn test_tokenize_keyword() {
        let tokens = tokenize("id f(x) = g(x);");
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Keyword && t.text == "id"));
    }
    
    #[test]
    fn test_tokenize_comment() {
        let tokens = tokenize("* This is a comment");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Comment);
    }
    
    #[test]
    fn test_tokenize_number() {
        let tokens = tokenize("x^10 + 2*y");
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Number && t.text == "10"));
        assert!(tokens.iter().any(|t| t.token_type == TokenType::Number && t.text == "2"));
    }
}
