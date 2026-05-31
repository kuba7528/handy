use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SpeechLanguage {
    English,
    Polish,
}

pub fn resolve_speech_language(selected_language: &str, app_language: &str) -> SpeechLanguage {
    let selected = selected_language.to_ascii_lowercase();
    if selected.starts_with("pl") {
        return SpeechLanguage::Polish;
    }
    if selected.starts_with("en") {
        return SpeechLanguage::English;
    }

    if app_language.to_ascii_lowercase().starts_with("pl") {
        SpeechLanguage::Polish
    } else {
        SpeechLanguage::English
    }
}

pub fn adjust_first_letter_case(text: &str, capitalize: bool) -> String {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };

    let replacement = if capitalize {
        first.to_uppercase().collect::<String>()
    } else {
        first.to_lowercase().collect::<String>()
    };

    format!("{replacement}{}", chars.as_str())
}

pub fn apply_spoken_numbers(text: &str, language: SpeechLanguage) -> String {
    let word_re = Regex::new(r"(?i)[\p{L}]+").expect("valid word regex");
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;
    let mut index = 0;
    let chars: Vec<(usize, usize, String)> = word_re
        .find_iter(text)
        .map(|m| (m.start(), m.end(), m.as_str().to_string()))
        .collect();

    while index < chars.len() {
        let (start, _, _) = chars[index];
        result.push_str(&text[last_end..start]);

        let mut run_end_index = index;
        while run_end_index < chars.len() {
            let (_, _, word) = &chars[run_end_index];
            if number_word_value(word, language).is_none() {
                break;
            }
            run_end_index += 1;
        }

        if run_end_index > index {
            let run_words: Vec<&str> = chars[index..run_end_index]
                .iter()
                .map(|(_, _, w)| w.as_str())
                .collect();

            if let Some(replacement) = parse_number_phrase(&run_words, language) {
                result.push_str(&replacement.to_string());
                last_end = chars[run_end_index - 1].1;
                index = run_end_index;
                continue;
            }
        }

        let (_, end, word) = &chars[index];
        result.push_str(word);
        last_end = *end;
        index += 1;
    }

    result.push_str(&text[last_end..]);
    result
}

pub fn apply_spoken_symbols(text: &str, language: SpeechLanguage) -> String {
    let mappings = symbol_mappings(language);
    let mut phrases: Vec<(&str, &str)> = mappings
        .iter()
        .map(|(phrase, symbol)| (phrase.as_str(), symbol.as_str()))
        .collect();
    phrases.sort_by_key(|(phrase, _)| std::cmp::Reverse(phrase.len()));

    let word_re = Regex::new(r"(?i)[\p{L}]+").expect("valid word regex");
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;

    for m in word_re.find_iter(text) {
        result.push_str(&text[last_end..m.start()]);

        let mut replaced = false;
        for (phrase, symbol) in &phrases {
            let phrase_words: Vec<&str> = phrase.split_whitespace().collect();
            if phrase_words.is_empty() {
                continue;
            }

            if phrase_words.len() == 1 {
                if tokens_equal(m.as_str(), phrase_words[0]) {
                    result.push_str(symbol);
                    replaced = true;
                    break;
                }
                continue;
            }

            let candidate = text[m.start()..]
                .split_whitespace()
                .take(phrase_words.len())
                .collect::<Vec<_>>();
            if candidate.len() == phrase_words.len()
                && candidate
                    .iter()
                    .zip(phrase_words.iter())
                    .all(|(a, b)| tokens_equal(a, b))
            {
                result.push_str(symbol);
                let consumed = candidate.join(" ");
                let consumed_len = text[m.start()..]
                    .find(&consumed)
                    .map(|offset| offset + consumed.len())
                    .unwrap_or(m.end() - m.start());
                last_end = m.start() + consumed_len;
                replaced = true;
                break;
            }
        }

        if !replaced {
            result.push_str(m.as_str());
            last_end = m.end();
        }
    }

    result.push_str(&text[last_end..]);
    result
}

fn number_word_value(word: &str, language: SpeechLanguage) -> Option<u64> {
    let normalized = normalize_token(word);
    match language {
        SpeechLanguage::English => english_number_words().get(normalized.as_str()).copied(),
        SpeechLanguage::Polish => polish_number_words().get(normalized.as_str()).copied(),
    }
}

fn normalize_token(word: &str) -> String {
    word.chars()
        .map(|c| match c {
            'ą' | 'Ą' => 'a',
            'ć' | 'Ć' => 'c',
            'ę' | 'Ę' => 'e',
            'ł' | 'Ł' => 'l',
            'ń' | 'Ń' => 'n',
            'ó' | 'Ó' => 'o',
            'ś' | 'Ś' => 's',
            'ź' | 'Ź' => 'z',
            'ż' | 'Ż' => 'z',
            other if other.is_ascii() => other.to_ascii_lowercase(),
            other => other,
        })
        .collect()
}

fn tokens_equal(a: &str, b: &str) -> bool {
    normalize_token(a) == normalize_token(b)
}

fn parse_number_phrase(words: &[&str], language: SpeechLanguage) -> Option<u64> {
    if words.is_empty() {
        return None;
    }

    if words
        .iter()
        .all(|word| number_word_value(word, language).is_some_and(|v| v <= 9))
    {
        let mut digits = String::new();
        for word in words {
            digits.push_str(&number_word_value(word, language)?.to_string());
        }
        return digits.parse().ok();
    }

    let mut total: u64 = 0;
    let mut current: u64 = 0;

    for word in words {
        let value = number_word_value(word, language)?;

        if value >= 1000 {
            if current == 0 {
                current = 1;
            }
            current *= value;
            total += current;
            current = 0;
            continue;
        }

        if value >= 100 {
            if current == 0 {
                current = 1;
            }
            current *= value;
            continue;
        }

        if value >= 20 {
            current += value;
            continue;
        }

        current += value;
    }

    let parsed = total + current;
    if parsed == 0 && words.len() == 1 {
        Some(0)
    } else if parsed > 0 || words.iter().any(|w| normalize_token(w) == "zero") {
        Some(parsed)
    } else {
        None
    }
}

fn english_number_words() -> HashMap<String, u64> {
    HashMap::from([
        ("zero".into(), 0),
        ("oh".into(), 0),
        ("one".into(), 1),
        ("two".into(), 2),
        ("three".into(), 3),
        ("four".into(), 4),
        ("five".into(), 5),
        ("six".into(), 6),
        ("seven".into(), 7),
        ("eight".into(), 8),
        ("nine".into(), 9),
        ("ten".into(), 10),
        ("eleven".into(), 11),
        ("twelve".into(), 12),
        ("thirteen".into(), 13),
        ("fourteen".into(), 14),
        ("fifteen".into(), 15),
        ("sixteen".into(), 16),
        ("seventeen".into(), 17),
        ("eighteen".into(), 18),
        ("nineteen".into(), 19),
        ("twenty".into(), 20),
        ("thirty".into(), 30),
        ("forty".into(), 40),
        ("fifty".into(), 50),
        ("sixty".into(), 60),
        ("seventy".into(), 70),
        ("eighty".into(), 80),
        ("ninety".into(), 90),
        ("hundred".into(), 100),
        ("thousand".into(), 1000),
    ])
}

fn polish_number_words() -> HashMap<String, u64> {
    HashMap::from([
        ("zero".into(), 0),
        ("jeden".into(), 1),
        ("jedna".into(), 1),
        ("jedno".into(), 1),
        ("dwa".into(), 2),
        ("dwie".into(), 2),
        ("trzy".into(), 3),
        ("cztery".into(), 4),
        ("piec".into(), 5),
        ("pięć".into(), 5),
        ("szesc".into(), 6),
        ("sześć".into(), 6),
        ("siedem".into(), 7),
        ("osiem".into(), 8),
        ("dziewiec".into(), 9),
        ("dziewięć".into(), 9),
        ("dziesiec".into(), 10),
        ("dziesięć".into(), 10),
        ("jedenascie".into(), 11),
        ("jedenaście".into(), 11),
        ("dwanascie".into(), 12),
        ("dwanaście".into(), 12),
        ("trzynascie".into(), 13),
        ("trzynaście".into(), 13),
        ("czternascie".into(), 14),
        ("czternaście".into(), 14),
        ("pietnascie".into(), 15),
        ("piętnaście".into(), 15),
        ("szesnascie".into(), 16),
        ("szesnaście".into(), 16),
        ("siedemnascie".into(), 17),
        ("siedemnaście".into(), 17),
        ("osiemnascie".into(), 18),
        ("osiemnaście".into(), 18),
        ("dziewietnascie".into(), 19),
        ("dziewiętnaście".into(), 19),
        ("dwadziescia".into(), 20),
        ("dwadzieścia".into(), 20),
        ("trzydziesci".into(), 30),
        ("trzydzieści".into(), 30),
        ("czterdziesci".into(), 40),
        ("czterdzieści".into(), 40),
        ("piecdziesiat".into(), 50),
        ("pięćdziesiąt".into(), 50),
        ("szescdziesiat".into(), 60),
        ("sześćdziesiąt".into(), 60),
        ("siedemdziesiat".into(), 70),
        ("siedemdziesiąt".into(), 70),
        ("osiemdziesiat".into(), 80),
        ("osiemdziesiąt".into(), 80),
        ("dziewiecdziesiat".into(), 90),
        ("dziewięćdziesiąt".into(), 90),
        ("sto".into(), 100),
        ("dwiescie".into(), 200),
        ("dwieście".into(), 200),
        ("trzysta".into(), 300),
        ("czterysta".into(), 400),
        ("piecset".into(), 500),
        ("pięćset".into(), 500),
        ("szescset".into(), 600),
        ("sześćset".into(), 600),
        ("siedemset".into(), 700),
        ("osiemset".into(), 800),
        ("dziewiecset".into(), 900),
        ("dziewięćset".into(), 900),
        ("tysiac".into(), 1000),
        ("tysiąc".into(), 1000),
    ])
}

fn english_symbol_map() -> HashMap<String, String> {
    HashMap::from([
        ("exclamation mark".into(), "!".into()),
        ("exclamation point".into(), "!".into()),
        ("exclamation".into(), "!".into()),
        ("at sign".into(), "@".into()),
        ("at symbol".into(), "@".into()),
        ("at".into(), "@".into()),
        ("number sign".into(), "#".into()),
        ("pound sign".into(), "#".into()),
        ("hash tag".into(), "#".into()),
        ("hashtag".into(), "#".into()),
        ("hash".into(), "#".into()),
        ("dollar sign".into(), "$".into()),
        ("dollar".into(), "$".into()),
        ("percent sign".into(), "%".into()),
        ("percent".into(), "%".into()),
        ("circumflex".into(), "^".into()),
        ("caret".into(), "^".into()),
        ("hat".into(), "^".into()),
        ("ampersand".into(), "&".into()),
        ("amp".into(), "&".into()),
        ("asterisk".into(), "*".into()),
        ("star".into(), "*".into()),
        ("open parenthesis".into(), "(".into()),
        ("close parenthesis".into(), ")".into()),
        ("open paren".into(), "(".into()),
        ("close paren".into(), ")".into()),
        ("left parenthesis".into(), "(".into()),
        ("right parenthesis".into(), ")".into()),
        ("left paren".into(), "(".into()),
        ("right paren".into(), ")".into()),
        ("opening parenthesis".into(), "(".into()),
        ("closing parenthesis".into(), ")".into()),
        ("underscore".into(), "_".into()),
        ("hyphen".into(), "-".into()),
        ("dash".into(), "-".into()),
        ("minus".into(), "-".into()),
        ("equals sign".into(), "=".into()),
        ("equal sign".into(), "=".into()),
        ("equals".into(), "=".into()),
        ("equal".into(), "=".into()),
        ("plus sign".into(), "+".into()),
        ("plus".into(), "+".into()),
        ("open square bracket".into(), "[".into()),
        ("close square bracket".into(), "]".into()),
        ("open bracket".into(), "[".into()),
        ("close bracket".into(), "]".into()),
        ("left square bracket".into(), "[".into()),
        ("right square bracket".into(), "]".into()),
        ("left bracket".into(), "[".into()),
        ("right bracket".into(), "]".into()),
        ("opening bracket".into(), "[".into()),
        ("closing bracket".into(), "]".into()),
        ("open curly brace".into(), "{".into()),
        ("close curly brace".into(), "}".into()),
        ("open brace".into(), "{".into()),
        ("close brace".into(), "}".into()),
        ("left curly brace".into(), "{".into()),
        ("right curly brace".into(), "}".into()),
        ("left brace".into(), "{".into()),
        ("right brace".into(), "}".into()),
        ("opening brace".into(), "{".into()),
        ("closing brace".into(), "}".into()),
        ("back slash".into(), "\\".into()),
        ("backslash".into(), "\\".into()),
        ("vertical bar".into(), "|".into()),
        ("vertical line".into(), "|".into()),
        ("pipe".into(), "|".into()),
        ("semicolon".into(), ";".into()),
        ("colon".into(), ":".into()),
        ("single quote".into(), "'".into()),
        ("apostrophe".into(), "'".into()),
        ("double quote".into(), "\"".into()),
        ("quotation mark".into(), "\"".into()),
        ("quote".into(), "\"".into()),
        ("less than sign".into(), "<".into()),
        ("less than".into(), "<".into()),
        ("left angle bracket".into(), "<".into()),
        ("open angle bracket".into(), "<".into()),
        ("greater than sign".into(), ">".into()),
        ("greater than".into(), ">".into()),
        ("right angle bracket".into(), ">".into()),
        ("close angle bracket".into(), ">".into()),
        ("question mark".into(), "?".into()),
        ("comma".into(), ",".into()),
        ("full stop".into(), ".".into()),
        ("period".into(), ".".into()),
        ("dot".into(), ".".into()),
        ("point".into(), ".".into()),
        ("forward slash".into(), "/".into()),
        ("slash".into(), "/".into()),
        ("tilde".into(), "~".into()),
        ("grave accent".into(), "`".into()),
        ("grave".into(), "`".into()),
        ("back tick".into(), "`".into()),
        ("backtick".into(), "`".into()),
    ])
}

fn polish_symbol_map() -> HashMap<String, String> {
    HashMap::from([
        ("wykrzyknik".into(), "!".into()),
        ("wykrzyk".into(), "!".into()),
        ("znak małpy".into(), "@".into()),
        ("małpa".into(), "@".into()),
        ("malpa".into(), "@".into()),
        ("at".into(), "@".into()),
        ("krzyżyk".into(), "#".into()),
        ("krzyzyk".into(), "#".into()),
        ("hasztag".into(), "#".into()),
        ("hash".into(), "#".into()),
        ("znak dolara".into(), "$".into()),
        ("dolar".into(), "$".into()),
        ("znak procenta".into(), "%".into()),
        ("procent".into(), "%".into()),
        ("daszek szybki".into(), "^".into()),
        ("daszek".into(), "^".into()),
        ("zykadełko".into(), "^".into()),
        ("zykadelko".into(), "^".into()),
        ("ampersand".into(), "&".into()),
        ("gwiazdka".into(), "*".into()),
        ("gwiazda".into(), "*".into()),
        ("asterisk".into(), "*".into()),
        ("nawias otwierający".into(), "(".into()),
        ("nawias otwarty".into(), "(".into()),
        ("nawias lewy".into(), "(".into()),
        ("nawias zamykający".into(), ")".into()),
        ("nawias zamknięty".into(), ")".into()),
        ("nawias zamkniety".into(), ")".into()),
        ("nawias prawy".into(), ")".into()),
        ("podkreślenie".into(), "_".into()),
        ("podkreslenie".into(), "_".into()),
        ("underscore".into(), "_".into()),
        ("myślnik".into(), "-".into()),
        ("myslnik".into(), "-".into()),
        ("minus".into(), "-".into()),
        ("kreska".into(), "-".into()),
        ("równa się".into(), "=".into()),
        ("rowna sie".into(), "=".into()),
        ("równa".into(), "=".into()),
        ("rowna".into(), "=".into()),
        ("znak plus".into(), "+".into()),
        ("plus".into(), "+".into()),
        ("nawias kwadratowy otwierający".into(), "[".into()),
        ("nawias kwadratowy zamykający".into(), "]".into()),
        ("nawias kwadratowy otwarty".into(), "[".into()),
        ("nawias kwadratowy zamknięty".into(), "]".into()),
        ("nawias kwadratowy zamkniety".into(), "]".into()),
        ("nawias kwadratowy lewy".into(), "[".into()),
        ("nawias kwadratowy prawy".into(), "]".into()),
        ("nawias klamrowy otwierający".into(), "{".into()),
        ("nawias klamrowy zamykający".into(), "}".into()),
        ("nawias klamrowy otwarty".into(), "{".into()),
        ("nawias klamrowy zamknięty".into(), "}".into()),
        ("nawias klamrowy zamkniety".into(), "}".into()),
        ("nawias klamrowy lewy".into(), "{".into()),
        ("nawias klamrowy prawy".into(), "}".into()),
        ("ukośnik odwrotny".into(), "\\".into()),
        ("ukosnik odwrotny".into(), "\\".into()),
        ("backslash".into(), "\\".into()),
        ("pionowa kreska".into(), "|".into()),
        ("kreska pionowa".into(), "|".into()),
        ("pipe".into(), "|".into()),
        ("średnik".into(), ";".into()),
        ("srednik".into(), ";".into()),
        ("dwukropek".into(), ":".into()),
        ("apostrof".into(), "'".into()),
        ("cudzysłów otwarty".into(), "\"".into()),
        ("cudzysłów".into(), "\"".into()),
        ("cudzyslow".into(), "\"".into()),
        ("mniejsze niż".into(), "<".into()),
        ("mniejsze niz".into(), "<".into()),
        ("mniejsze".into(), "<".into()),
        ("znak mniejszości".into(), "<".into()),
        ("znak mniejszosci".into(), "<".into()),
        ("większe niż".into(), ">".into()),
        ("wieksze niz".into(), ">".into()),
        ("większe".into(), ">".into()),
        ("wieksze".into(), ">".into()),
        ("znak większości".into(), ">".into()),
        ("znak wiekszosci".into(), ">".into()),
        ("znak zapytania".into(), "?".into()),
        ("pytajnik".into(), "?".into()),
        ("przecinek".into(), ",".into()),
        ("kropka na końcu".into(), ".".into()),
        ("kropka".into(), ".".into()),
        ("ukośnik".into(), "/".into()),
        ("ukosnik".into(), "/".into()),
        ("slash".into(), "/".into()),
        ("tylda".into(), "~".into()),
        ("grawis".into(), "`".into()),
        ("backtick".into(), "`".into()),
    ])
}

fn symbol_mappings(language: SpeechLanguage) -> HashMap<String, String> {
    match language {
        SpeechLanguage::English => english_symbol_map(),
        SpeechLanguage::Polish => polish_symbol_map(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_english_digit_sequence() {
        let result = apply_spoken_numbers("one two three", SpeechLanguage::English);
        assert_eq!(result, "123");
    }

    #[test]
    fn converts_polish_compound_number() {
        let result = apply_spoken_numbers("sto dwadzieścia trzy", SpeechLanguage::Polish);
        assert_eq!(result, "123");
    }

    #[test]
    fn converts_english_symbols() {
        let result = apply_spoken_symbols("email at example dot com", SpeechLanguage::English);
        assert_eq!(result, "email @ example . com");
    }

    #[test]
    fn converts_polish_symbols() {
        let result = apply_spoken_symbols("pięć procent", SpeechLanguage::Polish);
        assert_eq!(result, "pięć %");
    }

    #[test]

    #[test]
    fn converts_english_special_symbols() {
        let result = apply_spoken_symbols(
            "hash dollar percent caret ampersand asterisk underscore equals plus",
            SpeechLanguage::English,
        );
        assert_eq!(result, "# $ % ^ & * _ = +");
    }

    #[test]
    fn converts_polish_brackets_and_punctuation() {
        let result = apply_spoken_symbols(
            "nawias kwadratowy otwierający x nawias kwadratowy zamykający tylda grawis",
            SpeechLanguage::Polish,
        );
        assert_eq!(result, "[ x ] ~ `" );
    }

    #[test]
    fn converts_polish_at_and_exclamation() {
        let result = apply_spoken_symbols("znak małpy test wykrzyknik", SpeechLanguage::Polish);
        assert_eq!(result, "@ test !");
    }

    fn adjusts_first_letter_case() {
        assert_eq!(adjust_first_letter_case("Hello", false), "hello");
        assert_eq!(adjust_first_letter_case("hello", true), "Hello");
    }
}
