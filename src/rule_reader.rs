//! Functions to read Suiron facts and rules from a file.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::*;

/// Loads a knowledge base with facts and rules from a file.
///
/// Reads facts and rules from a source file, parses them to produce Rules,
/// then adds these Rules to the knowledge base.
///
/// If a parsing error occurs, the function returns an error message which
/// includes the offending line.
///
/// # Arguments
/// * knowledge base
/// * file_name
/// # Return
/// error message or None
/// # Usage
/// ```
/// use suiron::*;
///
/// let mut kb = KnowledgeBase::new();
/// let result = load_kb_from_file(&mut kb, "./tests/kings.txt");
/// match result {
///     Some(err) => { println!("{}", err); },
///     None => { // All OK.
///         print_kb(&kb);
///     },
/// }
/// // Should print out knowledge base.
/// ```
pub fn load_kb_from_file(kb: &mut KnowledgeBase, file_name: &str) -> Option<String> {

    let rules: Vec<String>;
    match read_facts_and_rules(file_name) {
        Ok(r) => { rules = r; },
        Err(error_message) => { return Some(error_message); },
    }

    let mut previous = "".to_string();

    for rule_str in rules {
        match parse_rule(&rule_str) {
            Ok(rule) => {
                previous = rule_str;
                add_rules!(kb, rule);
            },
            Err(msg) => {
                let error_message = load_parse_error(msg, previous);
                return Some(error_message); 
            },
        }
    }
    return None;

} // load_kb_from_file


/// Produces a parsing error message, which includes the previous line.
///
/// # Arguments
/// * error message
/// * previous line
/// # Return
/// * new error message
fn load_parse_error(err: String, previous_line: String) -> String {
    if previous_line.len() == 0 {
        return format!("{} {}", err, "Check start of file.");
    }
    return format!("{} Error occurs after: {}", err, previous_line);
} // load_parse_error


/// Reads Suiron facts and rules from a text file.
///
/// * Strips out all comments. (Comment delimiters are: #, % and // .)
/// * Checks for end-of-line issues.
/// * Checks for unmatched parentheses and brackets.
///
/// # Arguments
/// * file_name
/// # Return
/// * vector of rules or error message
/// # Usage
/// ```
/// use suiron::*;
///
/// match read_facts_and_rules("./tests/kings.txt") {
///     Ok(rules) => { println!("{:?}", rules); },
///     Err(parsing_error) => { println!("{}", parsing_error); },
/// }
/// // Prints out: ["male(Godwin).", "male(Tostig).",  ...
/// ```
pub fn read_facts_and_rules(file_name: &str) -> Result<Vec<String>, String> {

    let mut long_line = "".to_string();
    let mut rules: Vec<String> = vec![];

    match line_reader(file_name) {
        Ok(lines) => {

            let mut line_number = 1;
            for line in lines {
                if let Ok(line) = line {
                    let line = strip_comments(&line);
                    if line.len() > 0 {
                        match check_last_char(&line, line_number) {
                            Some(msg) => { return Err(msg); },
                            None => { long_line += &line; },
                        }
                        rules.push(line);
                    }
                }
                line_number += 1;
            }
            separate_rules(&long_line)
        },
        Err(msg) => {
            // Add file name to error message.
            let msg = format!("{}: {}", msg, file_name);
            return Err(msg);
        },
    } // match

} // read_facts_and_rules

/// Creates an iterator which reads lines from a file.
///
/// # Arguments
/// * file_name
/// # Return
/// * line reader
/// # Reference
/// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
///
fn line_reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Strips comments from a line.
///
/// In Suiron, valid comment delimiters are %, # and //.
///
/// <blockquote>
/// $In = [$H1, $H2 | $T], &nbsp; &nbsp; % This is a comment.
/// </blockquote>
///
/// Any text which occurs after these delimiters is considered a comment,
/// and removed from the line. However, if these delimiters occur within
/// braces, they are not treated as comment delimiters.
/// For example, in the line
///
/// <blockquote>
///    print(Your rank is %s., $Rank), &nbsp; &nbsp; % Print user's rank.
/// </blockquote>
///
/// the first percent sign does not start a comment, but the second
/// one does.
///
/// # Arguments
/// * `original line`
/// # Return
/// * `line without comments`
fn strip_comments(line: &str) -> String {

    let mut previous = 'x';
    let mut round_depth  = 0;
    let mut square_depth = 0;

    let mut index = 0;
    let mut has_comment = false;

    let chrs = str_to_chars!(line);
    for (i, ch) in chrs.iter().enumerate() {
        if *ch == '(' { round_depth += 1; }
        else if *ch == '[' { square_depth += 1; }
        else if *ch == ')' { round_depth -= 1; }
        else if *ch == ']' { square_depth -= 1; }
        else if round_depth == 0 && square_depth == 0 {
            if *ch == '#' || *ch == '%' {
                index = i;
                has_comment = true;
                break;
            } else if *ch == '/' && previous == '/' {
                index = i - 1;
                has_comment = true;
                break;
            }
        }
        previous = *ch;
    }

    if has_comment {
        return chars_to_string!(chrs[0..index]).trim().to_string();
    }
    else {
        return chars_to_string!(chrs).trim().to_string();
    }

}  // strip_comments

/// Divides a text string into a list of facts and rules.
///
/// Each rule or fact ends with a period.
///
/// # Arguments
/// * `text` - one long line
/// # Return
/// * `Result` - Ok(list of facts/rules) or Err(message)
///
fn separate_rules(text: &str) -> Result<Vec<String>, String> {

    let mut rule_str = "".to_string();
    let mut rules: Vec<String> = vec![];

    let mut round_depth: i32  = 0;
    let mut square_depth: i32 = 0;
    let mut num_quotes  = 0;

    let chrs = str_to_chars!(text);
    for ch in chrs {
        rule_str.push(ch);
        if ch == '.' && round_depth == 0 &&
            square_depth == 0 && num_quotes % 2 == 0 {
            rules.push(rule_str);
            rule_str = "".to_string();
        }
        else if ch == '(' { round_depth += 1; }
        else if ch == '[' { square_depth += 1; }
        else if ch == ')' { round_depth -= 1; }
        else if ch == ']' { square_depth -= 1; }
        else if ch == '"' { num_quotes += 1; }
    } // for

    // Check for unmatched brackets here.
    match unmatched_bracket(&rule_str, round_depth, square_depth) {
        None => {},
        Some(msg) => { return Err(msg); },
    }

    return Ok(rules);

} // separate_rules

/// Check that a line ends with a valid character.
///
/// In Suiron source code, facts and rules can be split over
/// several lines. For example, a rule can be formatted as:
///
/// <pre>
/// parse($In, $Out) :-
///     words_to_pos($In, $In2),
///     remove_punctuation($In2, $In3),
///     sentence($In3, $Out).
/// </pre>
///
/// The lines above end in dash, comma, comma and period,
/// which are valid. If a line were a simple word, such as:
///
/// <pre>
/// sentence
/// </pre>
///
/// That would indicate an error in the source.
///
/// Currently, valid characters are dash, comma, semicolon,
/// period and the equal sign.
///
/// # Arguments
/// * `line` - &str
/// * `line number`
/// # Return
/// * `Option` - Some(error message) or None
fn check_last_char(line: &str, num: usize) -> Option<String> {
    let chrs = str_to_chars!(line);
    let length = chrs.len();
    if length > 0 {
        let last = chrs[length - 1];
        if last != '-' && last != ',' && last != '.' &&
           last != '=' && last != ';' {
            let msg = format!("Check end of line {}: {}", num, line);
            return Some(msg);
        }
    }
    return None;   // No errors.
} // check_last_char()

/// Produces an error message for an unmatched bracket.
///
/// # Arguments
/// * `error line`
/// * `depth of round brackets`
/// * `depth of square brackets`
/// # Return
/// * `Option` - Some(error message) or None
///
fn unmatched_bracket(error_line: &str,
                     round_depth: i32,
                     square_depth: i32) -> Option<String> {

    // If no error, return None.
    if round_depth == 0 && square_depth == 0 { return None; }

    let mut msg = "";
    let msg2: String;

    if round_depth > 0       { msg = "Unmatched parenthesis: ("; }
    else if round_depth < 0  { msg = "Unmatched parenthesis: )"; }
    else if square_depth > 0 { msg = "Unmatched bracket: ["; }
    else if square_depth < 0 { msg = "Unmatched bracket: ]"; }
    let msg = msg.to_string();

    let chrs = str_to_chars!(error_line);

    if chrs.len() == 0 {
        msg2 = "Check start of file.".to_string();
    } else {
        let s = trim_error_line(&chrs);
        msg2 = "Check: ".to_string() + &s;
    }

    let msg = format!("{}\n{}", msg, msg2);
    return Some(msg);

} // unmatched_bracket

/// Trims a line at the first period or at 60 characters.
///
/// # Arguments
/// * `chrs` - a vector of characters
/// # Return
/// * `trimmed string`
fn trim_error_line(chrs: &Vec<char>) -> String {
    let mut index = 0;
    for ch in chrs {
        if *ch == '.' {
            index += 1;
            break;
        }
        if index == 100 { break; }
        index += 1;
    }
    return chars_to_string!(chrs[0..index]);
} // trim_error_line()

#[cfg(test)]
mod test {

    use super::*;

    // Produce an error message for unmatched parentheses and brackets.
    #[test]
    fn test_unmatched_bracket() {

        match unmatched_bracket("a_fact(a, b, c).", 0, 0) {
            Some(msg) => {
                let msg = format!("There should be no error message: {}", msg);
                panic!("{}", msg);
            },
            None => {},
        }
        match unmatched_bracket("a_fact(a, b, c.", 1, 0) {
            Some(msg) => { assert_eq!("Unmatched parenthesis: (\n\
                           Check: a_fact(a, b, c.", msg); },
            None => { panic!("Should produce an error message."); },
        }
        match unmatched_bracket("a, b, c]", 0, -1) {
            Some(msg) => { assert_eq!("Unmatched bracket: ]\n\
                           Check: a, b, c]", msg); },
            None => { panic!("Should produce an error message."); },
        }
    } // test_unmatched_bracket()

    // Produce an error message for lines which ends with an invalid character.
    #[test]
    fn test_check_last_char() {
        match check_last_char("mother($M, $C);", 10) {
            Some(msg) => {
                let msg = format!("There should be no error message: {}", msg);
                panic!("{}", msg);
            },
            None => {},
        }
        match check_last_char("mother($M, $C)", 10) {
            Some(msg) => { assert_eq!("Check end of line 10: mother($M, $C)", msg); },
            None => { panic!("Should produce an error message."); },
        }
    } // test_check_last_char()

    // Remove comments from lines.
    // Valid comment delimiters are: %, #, //
    #[test]
    fn test_strip_comments() {

        let line1 = "# This is a comment.";
        let line2 = "% Second comment.";
        let line3 = "// Third comment.";
        let line4 = "remove_punc([$H | $T], [$H | $T2]) :- \
             remove_punc($T, $T2). % Comment.";

        let line = strip_comments(line1);
        assert_eq!(line, "");
        let line = strip_comments(line2);
        assert_eq!(line, "");
        let line = strip_comments(line3);
        assert_eq!(line, "");
        let line = strip_comments(line4);
        assert_eq!(line, "remove_punc([$H | $T], [$H | $T2]) :- remove_punc($T, $T2).");

    } // test_strip_comments()

    // Divides a string of facts and rules into a vector of strings.
    // One fact/rule per entry. separate_rules() may also generate errors.
    #[test]
    fn test_separate_rules() {

        let text = "sibling($X, $Y) :- mother($Z, $X), mother($Z, $Y), !.\
            mother(Necessity, Invention). mother(Necessity, Innovation).";

        let expected = "[\"sibling($X, $Y) :- mother($Z, $X), mother($Z, $Y), !.\", \
                         \"mother(Necessity, Invention).\", \
                         \" mother(Necessity, Innovation).\"]";

        let bad_text = "sibling($X, $Y) :- mother($Z, $X), mother($Z, $Y), !.\
                mother(Necessity, Invention. mother(Necessity, Innovation).";

        let err_message = "Unmatched parenthesis: (\n\
                           Check: mother(Necessity, Invention.";

        match separate_rules(text) {
            Ok(rules) => {
                let s = format!("{:?}", rules);
                assert_eq!(expected, s);
            },
            Err(_) => { panic!("There should be no error here."); },
        }

        match separate_rules(bad_text) {
            Ok(_) => { panic!("Missing parenthesis should cause an error."); },
            Err(msg) => { assert_eq!(err_message, msg); },
        }
    } // test_separate_rules()

    // Test read_facts_and_rules() with invalid filename.
    #[test]
    fn test_invalid_filename() {
        let filename = "non-existent-file.txt";
        let rules = read_facts_and_rules(filename);
        match rules {
            Ok(_rules) => { panic!("The file {} should not exist.", filename); },
            Err(msg) => {
                if !msg.contains("No such file") {
                    panic!("Invalid error message: {}", msg);
                }
            },
        }
    } // test_invalid_filename

    // Reads Suiron facts and rules from a text file.
    // Confirm that the correct number of rules were read.
    // Check to ensure that the last rule is correct.
    #[test]
    fn test_read_facts_and_rules() {

        let rules = read_facts_and_rules("tests/kings.txt");
        match rules {
            Ok(rules) => {
                let n = rules.len();
                assert_eq!(n, 24, "Must read all rules.");
                let last_rule = &rules[n - 1];
                let s = format!("{}", last_rule);
                let s2 = "grandfather($X, $Y) :- parent($X, $Z), \
                          parent($Z, $Y), male($X).";
                assert_eq!(s, s2);
            },
            Err(msg) => { println!("{}", msg); },
        }

        let rules = read_facts_and_rules("tests/badrule1.txt");
        match rules {
            Ok(_) => { panic!("Should produce error message."); },
            Err(msg) => {
                let s = "Unmatched parenthesis: (\n\
                         Check: father($X, $Y) :- parent($X, $Y, male($X).";
                assert_eq!(s, msg);
            },
        }

        let rules = read_facts_and_rules("tests/badrule2.txt");
        match rules {
            Ok(_) => { panic!("Should produce error message."); },
            Err(msg) => {
                let s = "Unmatched parenthesis: )\n\
                         Check: mother($X, $Y) :- parent($X, $Y), female$X).";
                assert_eq!(s, msg);
            },
        }

        let rules = read_facts_and_rules("tests/badrule3.txt");
        match rules {
            Ok(_) => { panic!("Should produce error message."); },
            Err(msg) => {
                let s = "Check end of line 3: par";
                assert_eq!(s, msg);
            },
        }
    } // test_read_facts_and_rules()

    #[test]
    fn test_load_kb_from_file() {
        let mut kb = KnowledgeBase::new();
        let result = load_kb_from_file(&mut kb, "./tests/kings.txt");
        match result {
            Some(err) => { panic!("Should be no errors: {}", err); },
            None => { // All OK.
                let n = count_rules(&kb, "female/1");
                assert_eq!(5, n, "Should be 5 facts.");
            },
        }
    } // test_load_kb_from_file()

    #[test]
    fn test_trim_error_line() {
        let s = "Just a sentence. This should be trimmed.";
        let chrs = str_to_chars!(s);
        let s2 = trim_error_line(&chrs);
        assert_eq!("Just a sentence.", s2, "Should trim at period.");
        let s = "Long sentence       123456789012345678901234567890\
                 12345678901234567890123456789012345678901234567890\
                 123456789012345678901234567890";
        let chrs = str_to_chars!(s);
        let s2 = trim_error_line(&chrs);
        assert_eq!(100, s2.len(), "Should trim at 100 characters.");
    }

} // test
