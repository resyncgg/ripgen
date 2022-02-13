use crate::{DomainComponents, WordlistIterator};

pub fn permute_words_transform<'domain>(
    domain_components: &'domain DomainComponents,
    words: WordlistIterator<'domain>,
) -> impl Iterator<Item = String> + 'domain {
    words
        .flat_map(move |word| {
            (0 .. domain_components.count()).map(move |idx| {
                let domain_elems = domain_components.all();
                // this is the domain but with the word injected into it
                let augmented_domain_components: Vec<&str> = [
                    &domain_elems[.. idx],
                    [*word].as_slice(),
                    &domain_elems[idx ..]
                ].concat();

                // combine back into the full domain string
                augmented_domain_components.join(".")
            })
        })
}