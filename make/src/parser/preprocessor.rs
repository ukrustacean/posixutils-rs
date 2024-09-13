use std::collections::HashMap;
use std::iter::Peekable;

struct Preprocessor;

fn skip_blank(letters: &mut Peekable<impl Iterator<Item = char>>) {
    loop {
        let Some(letter) = letters.peek() else { break };
        if !letter.is_whitespace() { break };
        letters.next();
    }
}

fn suitable_ident(c: &char) -> bool { c.is_alphanumeric() || matches!(c, '_' | '.') }

fn get_ident(letters: &mut Peekable<impl Iterator<Item = char>>) -> String {

    let mut ident = String::with_capacity(10);

    // TODO: Remove unwrap
    loop {
        let Some(letter) = letters.peek() else { break };
        if !suitable_ident(letter) { break };
        println!("{letter}");
        ident.push(letters.next().unwrap());
    }

    ident
}

fn take_till_newline(letters: &mut Peekable<impl Iterator<Item = char>>) -> String {
    let mut content = String::with_capacity(20);

    while !matches!(letters.peek(), Some('\n') | Some('#')) {
        let Some(letter) = letters.next() else { break };
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
        let Some('=') = text.next() else { panic!("Expected `equals` sign after the macro name `{}`", macro_name) };
        skip_blank(&mut text);
        let macro_body = take_till_newline(&mut text);

        macro_table.insert(macro_name, macro_body);
    }

    macro_table
}

pub fn preprocess(source: &str) -> String {
    let mut source = source.to_string();
    
    loop {
        let mut substitutions = 0;
        let mut result = String::with_capacity(source.len());
        let table = generate_macro_table(&source);

        let mut letters = source.chars().peekable();
        loop {
            let Some(letter) = letters.next() else { break };
            if letter != '$' {
                result.push(letter);
                continue;
            }

            // TODO: Make proper error handling
            let Some(letter) = letters.next() else { panic!("Unexpected EOF after `$` symbol"); };
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
                    let Some(macro_body) = table.get(&c.to_string()) else { panic!("Undefined macro `{}`", c) };
                    result.push_str(macro_body);
                    substitutions += 1;
                    continue;
                }
                '(' | '{' => {
                    skip_blank(&mut letters);
                    let macro_name = get_ident(&mut letters);
                    skip_blank(&mut letters);
                    let Some(finilizer) = letters.next() else { panic!("Unexpected EOF at the end of macro expansion") };
                    if !matches!(finilizer, ')' | '}') { panic!("Unexpected `{}` at the end of macro expansion", finilizer) }

                    let Some(macro_body) = table.get(&macro_name) else { panic!("Undefined macro `{}`", macro_name) };
                    result.push_str(macro_body);
                    substitutions += 1;

                    continue;
                }
                // TODO: Make proper error handling
                c => { panic!("Unexpected `{}` after `$` symbol", c); }
            }
        }

        if substitutions == 0 { break result; } else { source = result; }
    }
}