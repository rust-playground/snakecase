use std::borrow::Cow;

const UNDERSCORE_CHAR: char = '_';

/// to_snakecase takes an argument which then is checked and if necessary modified into ensuring the text is in snakecase format.
pub fn to_snakecase<'a, S>(s: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    let input = s.into();
    let mut chars = input.char_indices().peekable();
    let mut has_lower = false;
    let mut has_underscore = false;
    let mut has_lower_since_underscore = false;

    while let Some((i, c)) = chars.next() {
        if c.is_lowercase() {
            has_lower = true;
            if has_underscore {
                has_lower_since_underscore = true;
            }
            continue;
        } else if c.is_numeric() {
            continue;
        } else if i > 0 && c == UNDERSCORE_CHAR {
            if let Some((_, c)) = chars.peek() {
                if c.is_lowercase() {
                    chars.next();
                    has_lower = true;
                    has_underscore = true;
                    has_lower_since_underscore = true;
                    continue;
                } else if c.is_numeric() {
                    chars.next();
                    has_underscore = true;
                    has_lower_since_underscore = false;
                    continue;
                }
            } else {
                // need to manipulate string '_' is the last character in the string
                // can return directly from here as we know to just strip the last char
                return Cow::Owned(input[..i].to_owned());
            }
        }

        // if we got here then we need to manipulate the string
        let mut result: String = String::with_capacity(input.len() + 5);
        result.push_str(&input[..i]);

        if c.is_uppercase() && (!has_lower || has_underscore && !has_lower_since_underscore) {
            result.extend(c.to_lowercase());
            peek_parse_valid(&mut result, &mut chars);
        } else if c.is_uppercase() {
            if !result.is_empty() {
                result.push(UNDERSCORE_CHAR);
            }
            result.extend(c.to_lowercase());
            peek_parse_valid(&mut result, &mut chars);
        }

        'outer: while let Some((_, c)) = chars.next() {
            if !c.is_alphanumeric() {
                continue;
            }
            if !result.is_empty() {
                result.push(UNDERSCORE_CHAR);
            }

            if c.is_uppercase() || c.is_numeric() {
                result.extend(c.to_lowercase());
                while let Some((_, c)) = chars.peek() {
                    if c.is_uppercase() || c.is_numeric() {
                        result.extend(c.to_lowercase());
                        chars.next();
                        continue;
                    }
                    peek_parse_lowercase_valid(&mut result, &mut chars);
                    continue 'outer;
                }
            }

            if c.is_lowercase() || c.is_numeric() {
                result.push(c);
                peek_parse_lowercase_valid(&mut result, &mut chars);
            }
        }
        return Cow::Owned(result);
    }
    input
}

#[inline(always)]
fn peek_parse_valid(
    result: &mut String,
    chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
) {
    while let Some((_, c)) = chars.peek() {
        if c.is_uppercase() || c.is_numeric() {
            result.extend(c.to_lowercase());
            chars.next();
            continue;
        }
        break;
    }
    peek_parse_lowercase_valid(result, chars);
}

#[inline(always)]
fn peek_parse_lowercase_valid(
    result: &mut String,
    chars: &mut std::iter::Peekable<std::str::CharIndices<'_>>,
) {
    while let Some((_, c)) = chars.peek() {
        if c.is_lowercase() || c.is_numeric() {
            result.push(*c);
            chars.next();
            continue;
        }
        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    macro_rules! snakecase_test {
        ($name:ident, $input:expr, $output:expr, $b:expr) => {
            #[test]
            fn $name() {
                let results = to_snakecase($input);
                assert_eq!(results, $output);
                assert_eq!(
                    match results {
                        Cow::Borrowed(_) => true,
                        _ => false,
                    },
                    $b
                );
            }
        };
    }

    snakecase_test!(empty, "", "", true);
    snakecase_test!(equal, "sample_text", "sample_text", true);
    snakecase_test!(space, "sample text", "sample_text", false);
    snakecase_test!(dash, "sample-text", "sample_text", false);
    snakecase_test!(multi_underscore, "sample___text", "sample_text", false);
    snakecase_test!(ending_underscore, "sample_text_", "sample_text", false);
    snakecase_test!(
        ending_multi_underscore,
        "sample_text__",
        "sample_text",
        false
    );
    snakecase_test!(uppercase_sep, "sampleText", "sample_text", false);
    snakecase_test!(
        multi_uppercase,
        "inviteYourCustomersAddInvites",
        "invite_your_customers_add_invites",
        false
    );
    snakecase_test!(
        space_with_uppercase,
        "sample 2 Text",
        "sample_2_text",
        false
    );
    snakecase_test!(special_chars, "FOO:BAR$BAZ", "foo_bar_baz", false);
    snakecase_test!(caps, "samPLE text", "sam_ple_text", false);
    snakecase_test!(
        multi_spaces,
        "   sample   2    Text   ",
        "sample_2_text",
        false
    );
    snakecase_test!(
        special_with_spaces,
        "   $#$sample   2    Text   ",
        "sample_2_text",
        false
    );
    snakecase_test!(caps_with_space_sep, "SAMPLE 2 TEXT", "sample_2_text", false);
    snakecase_test!(
        leading_underscore_special,
        "___$$Base64Encode",
        "base64_encode",
        false
    );
    snakecase_test!(caps_hash_sep, "FOO#BAR#BAZ", "foo_bar_baz", false);
    snakecase_test!(domain, "something.com", "something_com", false);
    snakecase_test!(
        special_leading_and_trailing,
        "$something%",
        "something",
        false
    );
    snakecase_test!(camel_case, "CStringRef", "cstring_ref", false);
    snakecase_test!(unicode_mixed, "ẞ•¶§ƒ˚foo˙∆˚¬", "ß_ƒ_foo", false);
    snakecase_test!(unicode_uppercase, "ẞ", "ß", false); // capitol unicode german to lowercase
    snakecase_test!(
        special_chars_long,
        "FOO:BAR$BAZ__Sample    Text___",
        "foo_bar_baz_sample_text",
        false
    );
    snakecase_test!(
        ascii_complex_random,
        "lk0B@bFmjrLQ_Z6YL",
        "lk0_b_b_fmjr_lq_z6yl",
        false
    );
    snakecase_test!(
        complex_random2,
        "@49L0S145_¬fwHƒ0TSLNVp",
        "49l0s145_fw_hƒ0_tslnvp",
        false
    );
    snakecase_test!(underscore_digit_uppercase, "_5TEst", "5test", false);
    snakecase_test!(end_caps, "edf_6N", "edf_6n", false);
    snakecase_test!(underscore_letter_upper, "f_pX9", "f_p_x9", false);
    snakecase_test!(underscore_letter_digit_upper, "p_z9Rg", "p_z9_rg", false);
}
