use regex::Regex;
use lazy_static::lazy_static;
use crate::{DomainComponents, WordlistIterator};

lazy_static! {
    static ref DIGIT_REGEX: Regex = Regex::new(r"\d{1,3}").unwrap();
}

#[inline(always)]
pub fn numbers_transform(
    domain_components: &DomainComponents,
    _: WordlistIterator
) -> impl Iterator<Item = String> {
    let domain_str: String = domain_components
        .subdomains()
        .join(".");

    let root = domain_components
        .root()
        .to_string();

    DIGIT_REGEX
        .captures_iter(&domain_str)
        .flat_map(|captures| {
            let detected_int_str = &captures[0];
            let detected_int: u64 = detected_int_str.parse().unwrap();
            let mut results = Vec::with_capacity(2);

            for offset in 1 .. 4 {
                if let Some(sub_int) = detected_int.checked_sub(offset) {
                    let replaced_front = domain_str.replace(detected_int_str, &sub_int.to_string());

                    results.push(format!("{}.{}", replaced_front, root));
                }

                if let Some(add_int) = detected_int.checked_add(offset) {
                    let replaced_front = domain_str.replace(detected_int_str, &add_int.to_string());

                    results.push(format!("{}.{}", replaced_front, root));
                }
            }

            results
        })
        .collect::<Vec<String>>()
        .into_iter()
}