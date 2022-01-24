use crate::{DomainComponents, WordlistIterator};

pub fn dash_transform<'domain>(
    domain_components: &'domain DomainComponents,
    words: WordlistIterator<'domain>,
) -> impl Iterator<Item = String> + 'domain {
    words.flat_map(move |word| {
        transform_components(domain_components, *word, dash)
            .chain(transform_components(domain_components, *word, rdash))
            .chain(transform_components(domain_components, *word, concat))
            .chain(transform_components(domain_components, *word, rconcat))
    })
}

fn transform_components<'domain, F: Fn(&str, &str) -> String + 'domain>(
    domain_components: &'domain DomainComponents<'domain>,
    word: &'domain str,
    transform: F
) -> impl Iterator<Item = String> + 'domain {
    (0 .. domain_components.count() - 1)
        .map(move |idx| {
            let new_word = transform(domain_components.all()[idx], word);

            let new_sub: &[&str] = &[
                &domain_components.all()[.. idx],
                [new_word.as_str()].as_slice(),
                &domain_components.all()[idx + 1 ..]
            ].concat();

            new_sub.join(".")
        })
}

fn dash(left: &str, right: &str) -> String {
    format!("{}-{}", left, right)
}

fn rdash(left: &str, right: &str) -> String {
    dash(right, left)
}

fn concat(left: &str, right: &str) -> String {
    format!("{}{}", left, right)
}

fn rconcat(left: &str, right: &str) -> String {
    concat(right, left)
}