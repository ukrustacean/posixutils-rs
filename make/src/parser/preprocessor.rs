struct Preprocessor;

pub fn preprocess(source: &str) -> String {
    let macro_defs = source.lines().filter(|line| line.contains("="));
    for def in macro_defs {
        println!("PP definition: {}", def);
    }

    "".into()
}