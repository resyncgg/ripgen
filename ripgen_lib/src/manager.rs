use crate::domain::DomainComponents;
use crate::error::RipGenError;
use fxhash::FxHashSet;
use std::slice::Iter as SliceIter;
use std::collections::hash_set::Iter as HashSetIter;
use crate::transform::RipGenTransform;

#[derive(Clone)]
/// Processes and manages domains and wordlist elements to enable creating [RipGenIterator](crate::RipGenIterator)
/// via [transformations](Self::transform).
pub struct RipGenManager<'domains> {
    domain_components: Vec<DomainComponents<'domains>>,
    elements: FxHashSet<&'domains str>,
}

impl<'domain> RipGenManager<'domain> {
    /// Creates a new `RipGenManager`.
    ///
    /// This can fail if any of the `domains` are unable to be parsed.
    pub fn new(
        domains: impl Iterator<Item=&'domain str>,
        words: impl Iterator<Item=&'domain str>,
        word_filter: &impl Fn(&&str) -> bool
    ) -> Result<RipGenManager<'domain>, RipGenError>
    {
        let domain_components: Vec<DomainComponents> = domains
            .filter(|line| !line.is_empty())
            .map(DomainComponents::try_from)
            .collect::<Result<_, _>>()?;

        let elements: FxHashSet<&'domain str> = crate::words::extract_words(domain_components.iter(), word_filter)
            .chain(words)
            .collect();

        let manager = RipGenManager {
            domain_components,
            elements
        };

        Ok(manager)
    }

    /// Begins a RipGen transform iterator.
    ///
    /// Requires a function that can take both a reference to a [DomainComponents](crate::DomainComponents)
    /// as well as an iterator that produces `&&str`.
    pub fn transform<'manager, F, O>(&'manager self, transform: F) -> RipGenTransform<'manager, 'domain, F, SliceIter<DomainComponents<'domain>>, HashSetIter<'manager, &'domain str>, O>
        where
            F: Fn(&'manager DomainComponents<'domain>, HashSetIter<'manager, &'domain str>) -> O,
            O: Iterator<Item = String>,
            'domain: 'manager
    {
        RipGenTransform::new(self.domain_components.iter(), self.elements.iter(), transform)
    }
}
