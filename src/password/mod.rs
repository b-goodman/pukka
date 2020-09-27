use rand::{thread_rng, Rng};
use std::collections::HashMap;

// Copied from https://passwordsgenerator.net/ - thanks!
// Include Symbols:( e.g. @#$% )
// Include Numbers:( e.g. 123456 )
// Include Lowercase Characters:( e.g. abcdefgh )
// Include Uppercase Characters:( e.g. ABCDEFGH )
// Exclude Similar Characters:( e.g. i, l, 1, L, o, 0, O )
// Exclude Ambiguous Characters:( { } [ ] ( ) / \ ' " ` ~ , ; : . < > )

// TODO - make this work, it would be cool.
// #[derive(Debug, Clone, Hash, PartialEq, Eq)]
// pub enum Options {
//     IncludeSymbols,
//     IncludeNumbers,
//     IncludeLowercase,
//     IncludeUppercase,
//     ExcludeSimilar,
//     ExcludeAmbiguous
// }

// impl Options {
//     pub fn as_key(&self) -> &'static str {
//         match *self {
//             Options::IncludeSymbols => "include_symbols",
//             Options::IncludeNumbers => "include_numbers",
//             Options::IncludeLowercase => "include_lowercase",
//             Options::IncludeUppercase => "include_uppercase",
//             Options::ExcludeSimilar => "exclude_similar",
//             Options::ExcludeAmbiguous => "exclude_ambiguous"
//         }
//     }
// }


pub fn generate(charset: &[u8], password_len: usize) -> String {
    let mut rng = thread_rng();
    (0..password_len).map(|_| {
        let idx = rng.gen_range(0, charset.len());
        charset[idx] as char
    }).collect()
}

// TODO - implement user options for generation.
pub fn from_options(user_options: &HashMap<&str, bool>) -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789)(*&^%$#@!~";

    let password_len: usize = 8;

    generate(charset, password_len)
}
