use rust_cli::cli::concat_text_file_and_command;


#[test]
fn test_concat_test_file_and_command() -> () {
    let str1 = "string1".to_string();
    let str2 = vec!["string2".to_string()];

    let out = concat_text_file_and_command(&str1, &str2);
    let expected = String::from("string1string2");

    assert_eq!(out, expected);
}