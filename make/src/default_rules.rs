#[derive(Debug)]
enum Suffix {
    O,
    C,
    Y,
    L,
    A,
    Sh,
    C_Tilde,
    Y_Tilde,
    L_Tilde,
    Sh_Tilde,
}

impl Suffix {
    fn from_extension(ext: &str) -> Option<Suffix> {
        match ext {
            ".o" => Some(Suffix::O),
            ".c" => Some(Suffix::C),
            ".y" => Some(Suffix::Y),
            ".l" => Some(Suffix::L),
            ".a" => Some(Suffix::A),
            ".sh" => Some(Suffix::Sh),
            ".c~" => Some(Suffix::C_Tilde),
            ".y~" => Some(Suffix::Y_Tilde),
            ".l~" => Some(Suffix::L_Tilde),
            ".sh~" => Some(Suffix::Sh_Tilde),
            _ => None,
        }
    }
}
