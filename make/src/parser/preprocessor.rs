use crate::parser::parse::Parse;
use crate::parser::SyntaxKind;
use std::collections::{HashMap, VecDeque};
use std::iter::Peekable;

fn skip_blank(letters: &mut Peekable<impl Iterator<Item = char>>) {
    while let Some(letter) = letters.peek() {
        if !letter.is_whitespace() {
            break;
        };
        letters.next();
    }
}

fn suitable_ident(c: &char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '.')
}

fn get_ident(letters: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut ident = String::new();

    while let Some(letter) = letters.peek() {
        if !suitable_ident(letter) {
            break;
        };
        ident.push(letter.clone());
        letters.next();
    }

    ident
}

fn take_till_eol(letters: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut content = String::new();

    while let Some(letter) = letters.peek() {
        if matches!(letter, '\n' | '#') {
            break;
        };
        content.push(letter.clone());
        letters.next();
    }

    content
}

fn generate_macro_table(source: &str) -> HashMap<String, String> {
    let macro_defs = source.lines().filter(|line| line.contains('='));
    let mut macro_table = HashMap::<String, String>::new();

    for def in macro_defs {
        let mut immediate = false;
        let mut text = def.chars().peekable();

        let macro_name = get_ident(&mut text);
        skip_blank(&mut text);
        let Some(symbol) = text.next() else {
            panic!("Unexpected end of line!")
        };
        match symbol {
            '=' => {}
            ':' => {
                let Some('=') = text.next() else {
                    panic!("Expected `=` after `:` in macro definition")
                };
                immediate = true;
            }
            c => panic!("Unexpected symbol `{}` in macro definition", c),
        };
        skip_blank(&mut text);
        let macro_body = take_till_eol(&mut text);

        macro_table.insert(
            macro_name,
            if immediate {
                substitute(&macro_body, &macro_table).0
            } else {
                macro_body
            },
        );
    }

    macro_table
}

fn substitute(source: &str, table: &HashMap<String, String>) -> (String, u32) {
    let mut substitutions = 0;
    let mut result = String::with_capacity(source.len());

    let mut letters = source.chars().peekable();
    while let Some(letter) = letters.next() {
        if letter != '$' {
            result.push(letter);
            continue;
        }

        // TODO: Make proper error handling
        let Some(letter) = letters.next() else {
            panic!("Unexpected EOF after `$` symbol");
        };
        match letter {
            // Internal macros - we leave them "as is"
            // yet as they will be dealt with in the
            // parsing stage with more context available
            c @ ('$' | '@' | '%' | '?' | '<' | '*') => {
                result.push('$');
                result.push(c);
                continue;
            }
            c if suitable_ident(&c) => {
                // TODO: Make proper error handling
                let Some(macro_body) = table.get(&c.to_string()) else {
                    panic!("Undefined macro `{}`", c)
                };
                result.push_str(macro_body);
                substitutions += 1;
                continue;
            }
            '(' | '{' => {
                skip_blank(&mut letters);
                let macro_name = get_ident(&mut letters);
                skip_blank(&mut letters);
                let Some(finilizer) = letters.next() else {
                    panic!("Unexpected EOF at the end of macro expansion")
                };
                if !matches!(finilizer, ')' | '}') {
                    panic!("Unexpected `{}` at the end of macro expansion", finilizer)
                }

                let Some(macro_body) = table.get(&macro_name) else {
                    panic!("Undefined macro `{}`", macro_name)
                };
                result.push_str(macro_body);
                substitutions += 1;

                continue;
            }
            // TODO: Make proper error handling
            c => {
                panic!("Unexpected `{}` after `$` symbol", c);
            }
        }
    }

    (result, substitutions)
}

pub fn preprocess(source: &str) -> String {
    let mut source = source.to_string();
    let table = generate_macro_table(&source);

    loop {
        let (result, substitutions) = substitute(&source, &table);
        if substitutions == 0 {
            break result;
        } else {
            source = result
        }
    }
}
