use regex::{Regex};
//use std::ops::Deref;



/// Use this struct to make customizations for your Slugify helper
/// You can use it to configure bad characters , or set the Slugify helper to collapse whitespaces or not.
/// 
#[derive(Debug, Clone)]
pub struct SluggifyOptions{
    collapse_whitespace: bool, //true by default
    disallowed_characters: Vec<char>
}

impl SluggifyOptions{
    /// Create a new SluggifyOptions configuration struct
    pub fn new() -> Self{
        SluggifyOptions { collapse_whitespace: true, 
        disallowed_characters: vec!['@', '#','$', '|', ']', '%', '!', '`', '~', '<','>',',','.', '+', '*', '&', '^', '?']
         }
    }
    /// specify to either collapses whitespaces or not
    /// default value is true
    pub fn set_collapse_whitespace(self, collapse_whitespace: bool) -> Self{
        SluggifyOptions {
            collapse_whitespace,
            ..self
        }
    }
    /// specify a list of bad characters to remove from the input string
    /// default value include the following characters ['@', '#','$', '|', ']', '%', '!', '`', '~', '<','>',',','.', '+', '*', '&', '^', '?']
    pub fn set_disallowed_characters(self, disallowed_characters: Vec<char>) -> Self{
        SluggifyOptions {
            disallowed_characters,
            ..self
        }
    }
    
}

///To create a slug version of any string
/// let expected = String::from("this-is-a-very-long-snake");
/// let output = sluggify("This is a very long snake", None);
/// assert_eq!(output, expected);
pub fn sluggify(input: &str, sluggify_options: Option<SluggifyOptions>) -> String {    
    let mut result_string = String::from("");
    let mut final_input = String::from("");
    let mut collapse_space: bool = false;
    let mut disallowed_chars: Vec<char> = vec![];
    let mut use_default_config: bool = false;
    match sluggify_options {
        Some(b) => {
            collapse_space = b.collapse_whitespace;
            disallowed_chars = b.disallowed_characters;
            use_default_config = false;
        },
        None => {
            use_default_config = true;
            collapse_space = true;
            disallowed_chars = SluggifyOptions::new().disallowed_characters;
        }
        
    };

    if use_default_config{
        let cfg = SluggifyOptions::new();
        collapse_space = cfg.collapse_whitespace;

        for c in input.chars() {
            if !cfg.disallowed_characters.contains(&c) {
                //final_input.push(c); 
                final_input = format!("{}{}", final_input, c);

            } else {
                //final_input.push(' ');
                final_input = format!("{}{}", final_input, c);

            }
        }

    }else{
        for c in input.chars() {
            if !disallowed_chars.contains(&c) {
                //final_input.push(c); 
                final_input = format!("{}{}", final_input, c);

            } else {
                //final_input.push(' ');
                final_input = format!("{}{}", final_input, ' ');

            }
        }

    }

    //we have still not handled collapse whitespaces...    
    if !collapse_space{
        //regex that leaves spaces or replace with -
        result_string = do_regex(&final_input, false);

    }else{
        //our normal regex...
        result_string = do_regex(&final_input, true);
    }

    return result_string;
    
}

fn do_regex(input: &str, collapse_spaces: bool) -> String {
    let mut re = match collapse_spaces {
        true => Regex::new(r"[^a-zA-Z0-9]+"),
        false => Regex::new(r"\W"),
    };
    let output = re.unwrap().replace_all(input, "-").to_string();
    output.trim_matches('-').to_lowercase()
}


