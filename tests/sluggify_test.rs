
#[cfg(test)]
mod sluggify_test{

    use sluggify::sluggify:: {SluggifyOptions, sluggify};
    //extern crate sluggify;

    #[test]
    fn test_sluggify(){
        //let expected = String::from("hello-world");
        let expected0 = String::from("this-is-a-very-long-slug-material");
        //assert_eq!(sluggify::sluggify("Hello World", None), expected);
        assert_eq!(sluggify("This is a very long slug, material", None), expected0);
    }
    #[test]
    fn test_sluggify_longer(){
        let expected = String::from("this-is-a-very-long-slug-material");
        assert_eq!(sluggify("This is a very long slug, material", None), expected);
    }

    #[test]
    fn test_sluggify_whitespace_collapse(){
        let original = "Jang et all";
        let expected = String::from("jang-et-all");
        let chars = vec!['*'];
        let ch = SluggifyOptions::new();

        let options = ch.set_disallowed_characters(chars);
        assert_eq!(sluggify(original, Some(options)), expected);
    }
    #[test]
    fn test_sluggify_longer1(){
        let expected = String::from("this-is-a-very-long-snake");
        assert_eq!(sluggify("This is a very long snake", None), expected);
    }

    #[test]
    fn test_sluggify_very_very_long_sentence_string(){
        let original = r#"I’ve tried to keep this library simple and accessible for beginners so you can easily use it in your own projects. If you are looking for a new feature or notice a bug open a PR or issue on the Github repository and I’ll try to get to it as quickly as possible."#;
        let expected = r#"i-ve-tried-to-keep-this-library-simple-and-accessible-for-beginners-so-you-can-easily-use-it-in-your-own-projects-if-you-are-looking-for-a-new-feature-or-notice-a-bug-open-a-pr-or-issue-on-the-github-repository-and-i-ll-try-to-get-to-it-as-quickly-as-possible"#;

        assert_eq!(sluggify(original, None), expected);

    }

    #[test]
    fn test_sluggify_whitespace_non_collapse(){
        let original = "a  b    \n  c   \t    d";
        let expected = "a--b-------c--------d";

        let ch = SluggifyOptions::new();

        let options = ch.set_collapse_whitespace(false);

        assert_eq!(sluggify(original, Some(options)), expected);
    }
    #[test]
    fn test_sluggify_special_char_whitespace_non_collapse(){
        let original = "a #$b    ** c   \t  \n  d";
        let expected = "a---b-------c---------d";//"a---b-------c---------d";

        let ch = SluggifyOptions::new();

        let options = ch.set_collapse_whitespace(false);

        assert_eq!(sluggify(original, Some(options)), expected);
    }
    #[test]
    fn test_sluggify_special_char_whitespace_collapse_plus_forbidden_characters(){
        let original = "a #$b    ** c   \t  \n  d";
        let expected = "a-b-c-d";//"a---b-------c---------d";
        let chars = vec!['*','$', '\t'];

        let options = SluggifyOptions::new().set_collapse_whitespace(true).set_disallowed_characters(chars);

        assert_eq!(sluggify(original, Some(options)), expected);
    }
    #[test]
    fn test_sluggify_special_char_whitespace_collapse(){
        let original = "a #$b    ** c   \t  \n  d";
        let expected = "a-b-c-d";

        let ch = SluggifyOptions::new();

        let options = ch.set_collapse_whitespace(true);

        assert_eq!(sluggify(original, Some(options)), expected);
    }



}