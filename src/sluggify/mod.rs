use regex::Regex;

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
    //println!("In the sluggify function... The sluggify option struct {:?}", sluggify_options);
    
    let mut result_string = String::from("");
    let mut final_input = String::from("");
    let mut collapse_space: bool = false;
    let mut disallowed_chars: Vec<char> = vec![];
    let mut use_default_config: bool = false;
    match sluggify_options {
        Some(b) => {
            collapse_space = b.collapse_whitespace;
            //println!("Collapsed spaces +++ {}", collapse_space);
            disallowed_chars = b.disallowed_characters;
            //println!("Disallowed.... +++ {:?}", &disallowed_chars);
            use_default_config = false;
            //println!("Use default config {}", use_default_config);

        },
        None => {
            use_default_config = true;
            collapse_space = true;
            disallowed_chars = SluggifyOptions::new().disallowed_characters;
            //println!("Use default config {}", use_default_config);
        }
        
    };

    // println!("Collapsed spaces....  +++ {}", collapse_space);
    //         println!("Disallowed.... +++ {:?}", &disallowed_chars);
    
    
    if use_default_config{
        let cfg = SluggifyOptions::new();
        collapse_space = cfg.collapse_whitespace;

        for c in input.chars() {
            if !cfg.disallowed_characters.contains(&c){
                final_input.push(c); 
            }else {
                final_input.push(' ');
            }
        }

    }else{
        for c in input.chars() {
            if !disallowed_chars.contains(&c){
                final_input.push(c); 
            }else{
                final_input.push(' ');
            }
        }

    }

    //we have still not handled collapse whitespaces...
    //println!("Final Input after processing through disallowed characters {}", &final_input);
    
    if !collapse_space{
        //regex that leaves spaces or replace with -
        result_string = do_regex(&final_input, false);
        //println!("Result String of Input {} is {}", &final_input, &result_string);

    }else{
        //our normal regex...
        result_string = do_regex(&final_input, true);
        //println!("Result String of Input {} is {}", &final_input, &result_string);
    }

    //println!("Result string for input {} is = {}",&input, &result_string);
    return result_string;
    
}

fn do_regex(input: &str, collapse_spaces: bool) -> String {
    let mut pattern: &str = "";

    let mut re: Regex = Regex::new("^/d{5}").unwrap();

    if collapse_spaces == true {
        re = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
    }else {
        re = Regex::new(r"\W").unwrap();
    }
    //println!("Patter for this input {} is{}", &input, &pattern);
    let output = re.replace_all(input, "-").to_string();
    output.trim_matches('-').to_lowercase()
}


