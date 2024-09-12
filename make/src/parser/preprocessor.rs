use std::collections::HashMap;
use std::iter::Peekable;

struct Preprocessor;

fn skip_blank(letters: &mut Peekable<impl Iterator<Item = char>>) {
    // TODO: Remove unwrap
    while letters.peek().unwrap().is_whitespace() {
        letters.next();
    }
}

fn suitable_ident(c: &char) -> bool { c.is_alphanumeric() || matches!(c, '_' | '.') }

fn get_ident(letters: &mut Peekable<impl Iterator<Item = char>>) -> String {

    let mut ident = String::with_capacity(10);

    // TODO: Remove unwrap
    while suitable_ident(letters.peek().unwrap()) {
        let Some(letter) = letters.next() else { continue };
        ident.push(letter);
    }

    ident
}

fn take_till_newline(letters: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut content = String::with_capacity(20);

    while !matches!(letters.peek(), Some('\n') | Some('#')) {
        let Some(letter) = letters.next() else { continue };
        content.push(letter);
    }

    content
}

fn generate_macro_table(source: &str) -> HashMap<String, String> {
    let macro_defs = source.lines().filter(|line| line.contains("="));
    let mut macro_table = HashMap::<String, String>::new();

    for def in macro_defs {
        let mut text = def.chars().peekable();

        let macro_name = get_ident(&mut text);
        skip_blank(&mut text);
        if text.next().unwrap() != '=' { panic!("Expected `equals` sign after the macro name `{}`", macro_name) }
        skip_blank(&mut text);
        let macro_body = take_till_newline(&mut text);

        macro_table.insert(macro_name, macro_body);
    }

    macro_table
}

pub fn preprocess(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let table = generate_macro_table(source);

    let mut letters = source.char_indices().peekable();
    let mut prev_idx = 0;
    loop {
        let Some((idx, letter)) = letters.next() else { break };
        if letter != '$' { continue; }

        result.push_str(&source[0..idx]);
        prev_idx = idx + 1;

        let Some((idx, letter)) = letters.next() else { break };
        if letter != '$' { continue; }

        if suitable_ident(&letter) { todo!("Sigle-char macros") }
        if !matches!(letter, '(' | '{') { todo!("Special-case macros") }

        skip_blank(&mut letters);
        let name = get_ident(&mut letters);
        skip_blank(&mut letters);

        let Some((idx, letter)) = letters.next() else { break };
        if !matches!(letter, ')' | '}') { panic!("Expected closed parenthesis or braces") }

        prev_idx = idx + 1;

        let Some(macro_body) = table.get(&name);
        result.push_str(macro_body);
    }

    result
}