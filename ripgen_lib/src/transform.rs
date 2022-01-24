use crate::{DomainComponents, RipGenIterator};

pub struct RipGenTransform<'manager, 'domain, F, D, W, O>
    where
        F: Fn(&'manager DomainComponents<'domain>, W) -> O,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        O: Iterator<Item = String>,
        'domain: 'manager
{
    domains: D,
    words: W,
    result_pool: Vec<String>,
    transform: F
}

impl<'manager, 'domain, F, D, W, O> RipGenTransform<'manager, 'domain, F, D, W, O>
    where
        F: Fn(&'manager DomainComponents<'domain>, W) -> O,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        O: Iterator<Item = String>,
        'domain: 'manager
{
    pub fn new(domains: D, words: W, transform: F) -> Self {
        Self {
            domains,
            words,
            result_pool: Vec::with_capacity(1024 * 4),
            transform
        }
    }
}


impl<'manager, 'domain, F, D, W, O> Iterator for RipGenTransform<'manager, 'domain, F, D, W, O>
    where
        F: Fn(&'manager DomainComponents<'domain>, W) -> O,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        O: Iterator<Item = String>,
        'domain: 'manager
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.result_pool.pop() {
            return Some(next);
        }

        loop {
            let domain = self.domains.next()?;

            self.result_pool.extend((self.transform)(domain, self.words.clone()));

            if let Some(result) = self.result_pool.pop() {
                return Some(result);
            }
        }
    }
}


impl<'manager, 'domain, F, D, W, O> RipGenIterator<'manager, 'domain, D, W> for RipGenTransform<'manager, 'domain, F, D, W, O>
    where
        F: Fn(&'manager DomainComponents<'domain>, W) -> O,
        D: Iterator<Item = &'manager DomainComponents<'domain>> + Clone,
        W: Iterator<Item = &'manager &'domain str> + Clone,
        O: Iterator<Item = String>,
        'domain: 'manager
{
    fn get_domains_iter(&self) -> D {
        self.domains.clone()
    }

    fn get_words_iter(&self) -> W {
        self.words.clone()
    }
}