use std::borrow::Cow;

const UNDERSCORE_BYTE: u8 = b'_';

/// to_snakecase takes an argument which then is checked and if necessary modified into ensuring the text is in snakecase format, unicode characters are discarded.
pub fn to_snakecase<'a, S>(s: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>,
{
    let input = s.into();
    let bytes = input.as_bytes();
    let mut idx = 0;
    let mut has_lower = false;
    let mut has_underscore = false;
    let mut has_lower_since_underscore = false;

    // loop through all good characters:
    // - lowercase
    // - digit
    // - underscore (as long as the next character is lowercase or digit)
    while idx < bytes.len() {
        if bytes[idx].is_ascii_lowercase() {
            idx += 1;
            has_lower = true;
            if has_underscore {
                has_lower_since_underscore = true;
            }
            continue;
        } else if bytes[idx].is_ascii_digit() {
            idx += 1;
            continue;
        } else if bytes[idx] == UNDERSCORE_BYTE
            && idx > 0
            && idx < bytes.len() - 1
            && is_lower_or_digit(bytes[idx + 1])
        {
            idx += 1;
            has_underscore = true;
            has_lower_since_underscore = false;
            continue;
        }
        break;
    }

    if idx >= bytes.len() {
        // '>=' performs much better than '==', I suspect it's due to bounds checking
        return input; // no changes needed, can just borrow the string
    }
    // if we get then we must need to manipulate the string
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len() + 5);
    result.extend_from_slice(&bytes[..idx]);

    if bytes[idx].is_ascii_uppercase()
        && (!has_lower || has_underscore && !has_lower_since_underscore)
    {
        while idx < bytes.len() && is_upper_or_digit_add(&mut result, bytes[idx]) {
            idx += 1;
        }

        while idx < bytes.len() && is_lower_or_digit(bytes[idx]) {
            result.push(bytes[idx]);
            idx += 1;
        }
    }

    while idx < bytes.len() {
        if !bytes[idx].is_ascii_alphanumeric() {
            idx += 1;
            continue;
        }

        if !result.is_empty() {
            result.push(UNDERSCORE_BYTE);
        }

        while idx < bytes.len() && is_upper_or_digit_add(&mut result, bytes[idx]) {
            idx += 1;
        }

        while idx < bytes.len() && is_lower_or_digit(bytes[idx]) {
            result.push(bytes[idx]);
            idx += 1;
        }
    }

    // we know this is safe because prior to this we eliminated all non-ascii chars so we are guaranteed
    // to only have utf-8 at this point.
    Cow::Owned(unsafe { String::from_utf8_unchecked(result) })
}

#[inline]
fn is_upper_or_digit_add(result: &mut Vec<u8>, b: u8) -> bool {
    if b.is_ascii_uppercase() {
        result.push(b.to_ascii_lowercase());
        true
    } else if b.is_ascii_digit() {
        result.push(b);
        true
    } else {
        false
    }
}

#[inline]
fn is_lower_or_digit(b: u8) -> bool {
    b.is_ascii_lowercase() || b.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ascii
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
    snakecase_test!(unicode_mixed, "ẞ•¶§ƒ˚foo˙∆˚¬", "foo", false);
    snakecase_test!(unicode_uppercase, "ẞ", "", false); // capitol unicode german to lowercase
    snakecase_test!(
        special_chars_long,
        "FOO:BAR$BAZ__Sample    Text___",
        "foo_bar_baz_sample_text",
        false
    );
    snakecase_test!(digit_underscore, "5test", "5test", true);
    snakecase_test!(character_digit, "test5", "test5", true);
    snakecase_test!(uppercase_digit, "THE5r", "the5r", false);
    snakecase_test!(digit_uppercase, "5TEst", "5test", false);
    snakecase_test!(starts_with_garbage, "@%#&5TEst", "5test", false);
    snakecase_test!(
        complex_random,
        "lk0B@bFmjrLQ_Z6YL",
        "lk0_b_b_fmjr_lq_z6yl",
        false
    );
    snakecase_test!(
        complex_random2,
        "@49L0S145_¬fwHƒ0TSLNVp",
        "49l0s145_fw_h_0tslnvp",
        false
    );
    snakecase_test!(underscore_digit_uppercase, "_5TEst", "5test", false);
    snakecase_test!(end_caps, "edf_6N", "edf_6n", false);
    snakecase_test!(underscore_letter_upper, "f_pX9", "f_p_x9", false);
    snakecase_test!(underscore_letter_digit_upper, "p_z9Rg", "p_z9_rg", false);
}
