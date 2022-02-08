use crate::{DomainComponents, WordlistIterator};

pub fn swap_word_transform<'domain>(
    domain_components: &'domain DomainComponents,
    words: WordlistIterator<'domain>,
) -> impl Iterator<Item = String> + 'domain {
    let root_string = domain_components.root().to_string();
    let subdomain_string: String = domain_components.subdomains().join(".");
    let subdomain_replace = subdomain_string.clone();

    let word_copy = words.clone();

    words
        .filter(move |word| subdomain_string.contains(*word))
        .flat_map(move |word| {
            let word_clone = <&str>::clone(word);
            let subdomain_replace = subdomain_replace.clone();
            let root_string = root_string.clone();

            word_copy
                .clone()
                .filter(move |sub_word| **sub_word != word_clone)
                .map(move |sub_word| {
                    let replaced_subdomain = subdomain_replace.replace(word, sub_word);
                    format!("{replaced_subdomain}.{root_string}")
                })
        })
}