use regex::Regex;

pub const REQUIRE_JA_TEL_REGEX: &str = r"(^0([0-9]-[0-9]{4}|[0-9]{2}-[0-9]{3}|[0-9]{3}-[0-9]{2}|[0-9]{4}-[0-9]{1})-[0-9]{4}$)|(^[0-9]{3}-[0-9]{4}-[0-9]{4}$)|(^0120-([0-9]-[0-9]{5}|[0-9]{2}-[0-9]{4}|[0-9]{3}-[0-9]{3}|[0-9]{4}-[0-9]{2}|[0-9]{5}-[0-9]|[0-9]{6})$)";
pub const OPTIONAL_HYPHEN_JA_TEL_REGEX: &str = r"(^0([0-9]-?[0-9]{4}|[0-9]{2}-?[0-9]{3}|[0-9]{3}-?[0-9]{2}|[0-9]{4}-?[0-9]{1})-?[0-9]{4}$)|(^[0-9]{3}-?[0-9]{4}-?[0-9]{4}$)|(^0120-?([0-9]-?[0-9]{5}|[0-9]{2}-?[0-9]{4}|[0-9]{3}-?[0-9]{3}|[0-9]{4}-?[0-9]{2}|[0-9]{5}-?[0-9]|[0-9]{6})$)";
pub const NO_HYPHEN_JA_TEL_REGEX: &str = r"^[0-9]{10,11}$";

pub enum HyphenCheck {
    Required,
    Optional,
    NoCheck,
}

pub fn japhonex(hyphen_check: HyphenCheck) -> regex::Regex {
    let r: regex::Regex;
    match hyphen_check {
        HyphenCheck::Required => r = Regex::new(REQUIRE_JA_TEL_REGEX).unwrap(),
        HyphenCheck::Optional => r = Regex::new(OPTIONAL_HYPHEN_JA_TEL_REGEX).unwrap(),
        HyphenCheck::NoCheck => r = Regex::new(NO_HYPHEN_JA_TEL_REGEX).unwrap(),
    }

    return r;
}

#[cfg(test)]
mod japhonex {
    use super::*;

    #[test]
    fn test_japhonex_optional() {
        let regex: regex::Regex = japhonex(HyphenCheck::Optional);
        let result = regex.is_match("000-0000-0000");

        assert_eq!(result, true);
    }

    #[test]
    fn test_japhonex_required() {
        let regex: regex::Regex = japhonex(HyphenCheck::Required);
        let pass_result = regex.is_match("000-0000-0000");
        let miss_result = regex.is_match("00000000000");

        assert_eq!(pass_result, true);
        assert_eq!(miss_result, false);
    }

    #[test]
    fn test_japhonex_no_hyphen() {
        let regex: regex::Regex = japhonex(HyphenCheck::NoCheck);
        let pass_result_1 = regex.is_match("000-0000-0000");
        let pass_result_2 = regex.is_match("00000000000");
        let miss_result = regex.is_match("000000000000000");

        assert_eq!(pass_result_1, false);
        assert_eq!(pass_result_2, true);
        assert_eq!(miss_result, false);
    }
}
