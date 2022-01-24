use std::marker::PhantomData;
use crate::DomainComponents;
use crate::transform::RipGenTransform;

/// Describes an iterator that can be apart of a [RipGenChain](crate::RipGenChain).
pub trait RipGenIterator<'manager, 'domain, D, W>
    where
        Self: Iterator<Item = String> + Sized,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        'domain: 'manager
{
    /// Returns an iterator over the underlying [DomainComponents](crate::DomainComponents).
    fn get_domains_iter(&self) -> D;

    /// Returns an iterator over the underlying wordlist.
    fn get_words_iter(&self) -> W;

    /// Chain this `RipGenIterator` with another `RipGenIterator` over the specified transform.
    ///
    /// ```
    /// # use ripgen_lib::{DomainComponents, RipGenIterator, RipGenManager, WordlistIterator};
    /// # fn add_prefix(word: impl Into<String>) -> impl Fn(&DomainComponents, WordlistIterator) -> std::vec::IntoIter<String> {
    /// #    let word_str = word.into();
    /// #    move |domain_components: &DomainComponents, _: WordlistIterator| -> std::vec::IntoIter<String> {
    /// #        let domain_str: String = domain_components
    /// #            .all()
    /// #            .join(".");
    /// #
    /// #        vec![format!("{}.{}", word_str, domain_str)].into_iter()
    /// #    }
    /// # }
    /// let domains = vec!["example.com"];
    /// let domain_iter = domains.iter().map(|elem| *elem);
    /// let wordlist = vec![];
    /// let wordlist_iter = wordlist.iter().map(|elem| *elem);
    ///
    /// let manager = RipGenManager::new(domain_iter, wordlist_iter)
    ///     .expect("Failed to parse domains.");
    ///
    /// let mut iter = manager
    ///     .transform(add_prefix("admin"))
    ///     .chain_transform(add_prefix("internal"))
    ///     .chain_transform(add_prefix("manager"));
    ///
    /// assert_eq!(iter.next(), Some("admin.example.com".to_string()));
    /// assert_eq!(iter.next(), Some("internal.example.com".to_string()));
    /// assert_eq!(iter.next(), Some("manager.example.com".to_string()));
    /// ```
    fn chain_transform<F, O>(self, transform: F) -> RipGenChain<'manager, 'domain, Self, RipGenTransform<'manager, 'domain, F, D, W, O>, D, W>
        where
            F: Fn(&'manager DomainComponents<'domain>, W) -> O,
            O: Iterator<Item = String>,
            'domain: 'manager
    {
        let domain_transform = RipGenTransform::new(self.get_domains_iter(), self.get_words_iter(), transform);

        RipGenChain::new(self, domain_transform)
    }
}

pub struct RipGenChain<'manager, 'domain, L, R, D, W>
    where
        L: RipGenIterator<'manager, 'domain, D, W>,
        R: RipGenIterator<'manager, 'domain, D, W>,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        'domain: 'manager
{
    left: Option<L>,
    right: Option<R>,
    manager_phantom: PhantomData<&'manager ()>,
    domain_phantom: PhantomData<&'domain ()>,
    domain_iterator_phantom: PhantomData<D>,
    word_iterator_phantom: PhantomData<W>
}

impl<'manager, 'domain, L, R, D, W> RipGenChain<'manager, 'domain, L, R, D, W>
    where
        L: RipGenIterator<'manager, 'domain, D, W>,
        R: RipGenIterator<'manager, 'domain, D, W>,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        'domain: 'manager
{
    fn new(left: L, right: R) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
            manager_phantom: PhantomData,
            domain_phantom: PhantomData,
            domain_iterator_phantom: PhantomData,
            word_iterator_phantom: PhantomData
        }
    }
}

impl<'manager, 'domain, L, R, D, W> Iterator for RipGenChain<'manager, 'domain, L, R, D, W>
    where
        L: RipGenIterator<'manager, 'domain, D, W>,
        R: RipGenIterator<'manager, 'domain, D, W>,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        'domain: 'manager
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut inner) = self.left {
            if let Some(output) = inner.next() {
                return Some(output);
            }

            self.left = None;
        }

        if let Some(ref mut inner) = self.right {
            if let Some(output) = inner.next() {
                return Some(output);
            }

            self.right = None;
        }

        None
    }
}

impl<'manager, 'domain, L, R, D, W> RipGenIterator<'manager, 'domain, D, W> for RipGenChain<'manager, 'domain, L, R, D, W>
    where
        L: RipGenIterator<'manager, 'domain, D, W>,
        R: RipGenIterator<'manager, 'domain, D, W>,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        'domain: 'manager
{
    fn get_domains_iter(&self) -> D {
        match self.left {
            Some(ref inner) => inner.get_domains_iter(),
            None => match self.right {
                Some(ref inner) => inner.get_domains_iter(),
                None => panic!("huh")
            }
        }
    }

    fn get_words_iter(&self) -> W {
        match self.left {
            Some(ref inner) => inner.get_words_iter(),
            None => match self.right {
                Some(ref inner) => inner.get_words_iter(),
                None => panic!("huh")
            }
        }
    }
}