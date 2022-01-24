use crate::{DomainComponents, WordlistIterator};

pub fn swap_word_transform<'domain>(
    domain_components: &'domain DomainComponents,
    words: WordlistIterator<'domain>,
) -> impl Iterator<Item = String> + 'domain {
    let domain_str: String = domain_components.all().join(".");
    let sub_str: String = domain_components.subdomains().join(".");

    let word_copy = words.clone();

    words
        .filter(|word| word.len() > 3)
        .filter(move |word| sub_str.contains(*word))
        .flat_map(move |word| {
            let word_clone = <&str>::clone(word);
            let inner_domain_str = domain_str.clone();

            word_copy
                .clone()
                .filter(move |sub_word| **sub_word != word_clone)
                .map(move |sub_word| inner_domain_str.clone().replace(word, sub_word))
        })
}