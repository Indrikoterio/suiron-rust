//! Functions which divide an English sentence into a list of words and punctuation.
//
// Cleve (Klivo) Lendon  2023

use suiron::*;

/// max: 120
const MAX_WORDS_IN_SENTENCE: usize = 120;

/// [', ", «, ‘, “]
const LEFT_QUOTES: [char; 5]  = ['\'', '"', '\u{00ab}', '\u{2018}', '\u{201c}'];
/// [', ", », ’, ”]
const RIGHT_QUOTES: [char; 5] = ['\'', '"', '\u{00bb}', '\u{2019}', '\u{201d}'];

/// Tests whether a character is an apostrophe.
///
/// # Arguments
/// * character to test
/// # Return
/// * true if character is an apostrophe
/// # Usage
/// ```
/// let ch = 'ʼ';
/// if is_an_apostrophe(ch) { println!("apostrophe"); }
/// // Prints: apostrophe
/// ```
fn is_an_apostrophe(ch: char) -> bool {
    if ch == '\'' || ch == '\u{02bc}' { return true; }
    return false;
}

/// Determines whether a character is punctuation.
///
/// EXCEPT if the character is a period (.).
/// A period could be part of an abbreviation or number (eg. 37.49).
///
/// # Arguments
/// * character to test
/// # Return
/// * true if character is punctuation
/// # Usage
/// ```
/// let ch = '&';
/// if is_punctuation(ch) { println!("ampersand"); }
/// // Prints: ampersand
/// ```
fn is_punctuation(ch: char) -> bool {
    if ch == '.' { return false; }
    if ch >= '!' && ch <= '/' { return true; }
    if ch >= ':' && ch <= '@' { return true; }
    if ch == '\u{2013}'         { return true; }  // en-dash
    if is_quote_mark(ch).is_some()   { return true; }
    return false;
}

/// Determines whether the character is a quote mark ("'«).
///
/// If yes, return the index of the quote mark in LEFT_QUOTES or RIGHT_QUOTES.
///
/// # Arguments
/// * character to test
/// # Return
/// * index of quote or None
/// # Usage
/// ```
/// let ch = '»';
/// if is_quote_mark(ch).is_some() { println!("quote"); }
/// // Prints: quote
/// ```
fn is_quote_mark(ch: char) -> Option<usize> {
    for i in 0..LEFT_QUOTES.len() {
        if ch == LEFT_QUOTES[i] { return Some(i); }
        if ch == RIGHT_QUOTES[i] { return Some(i); }
    }
    return None;
} // is_quote_mark

/// Determines whether a period is at the end of a sentence.
///
/// (If it is at the end, it must be punctuation.)
///
/// # Arguments
/// * sentence (vector of chars)
/// * index
/// # Return
/// * bool - true if end of sentence
/// # Usage
/// ```
/// let sentence = "The value of π is 3.14.  ";
/// let chrs = str_to_chars!(sentence);
/// println!("{}", end_of_sentence(&chrs, 19)); // Prints: false
/// println!("{}", end_of_sentence(&chrs, 22)); // Prints: true
/// ```
pub fn end_of_sentence(sentence: &Vec<char>, index: usize) -> bool {
    let mut index = index;
    let length = sentence.len();
    if index >= length - 1 { return true; }
    while index < length {
        let ch = sentence[index];
        index += 1;
        if letter_number_hyphen(ch) { return false; }
    }
    return true;
} // end_of_sentence

/// Divides a sentence into a vector of words and punctuation.
///
/// # Arguments
/// * sentence string
/// # Return
/// * vector of words and punctuation
/// # Usage
/// ```
/// let words = get_words("Is you is or is you ain't?");
/// println!("{:?}", words);
/// // Prints: ["Is", "you", "is", "or", "is", "you", "ain't", "?"]
/// ```
fn get_words(sentence: &str) -> Vec<String> {

    let mut words: Vec<String> = Vec::new();
    let mut number_of_words: usize = 0;

    let chrs = str_to_chars!(sentence);
    let length = chrs.len();

    let mut start_index: usize = 0;
    let mut last_index: usize;

    while start_index < length && number_of_words < MAX_WORDS_IN_SENTENCE {

        let mut character = ' ';

        // Skip spaces, etc.
        while start_index < length {
            character = chrs[start_index];
            if character > ' ' { break; }
            start_index += 1;
        }
        if start_index >= length { break; }

        // A period at the end of a sentence is punctuation.
        // A period in the middle is probably part of an abbreviation
        // or number, eg.: 7.3
        if character == '.' && end_of_sentence(&chrs, start_index) {
            words.push(".".to_string());
            start_index += 1;
        } else if is_punctuation(character) {
            words.push(format!("{}", character));
            start_index += 1;
        } else if letter_number_hyphen(character) {

            last_index = start_index + 1;
            while last_index < length {
                character = chrs[last_index];
                if character == '.' {
                    if end_of_sentence(&chrs, last_index) { break; }
                    // There might be an apostrophe within the word: don't, we've
                } else if is_an_apostrophe(character) {
                    if last_index < length - 1 {
                        let ch2 = chrs[last_index + 1];
                        if !letter_number_hyphen(ch2) { break; }
                    }
                } else {
                    if !letter_number_hyphen(character) { break; }
                }
                last_index += 1;
            } // while

            let word = &chrs[start_index..last_index];
            words.push(chars_to_string!(word));

            number_of_words += 1;

            start_index = last_index;

        } else {  // unknown character.
            start_index += 1;
        }
    } // while

    return words;

} // get_words

/// Divides a sentence into words.
///
/// # Argument
/// * original sentence
/// # Return
/// * vector of words
/// # Usage
/// ```
/// let sentence = "   I know\nmy\nrights.   ";
/// let words = sentence_to_words(sentence);
/// println!("{:?}", words);
/// // Prints: ["I", "know", "my", "rights", "."]
/// ```
pub fn sentence_to_words(sentence: &str) -> Vec<String> {
    // Clean up the string. New line becomes a space.
    let s = sentence.replace("\n", " ");
    // Divide string into words and punctuation.
    return get_words(&s);
} // sentence_to_words()
