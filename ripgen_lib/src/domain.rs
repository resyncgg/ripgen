use addr::dns::Name;
use addr::parse_dns_name;
use crate::error::RipGenError;

#[derive(Clone)]
/// Contains the byproduct of parsing a domain
pub struct DomainComponents<'domain> {
    components: Vec<&'domain str>
}

impl<'domain> DomainComponents<'domain> {
    /// Returns an iterator that contains all of the names in the subdomains.
    pub fn subdomains_iter(&self) -> impl Iterator<Item = &&'domain str> {
        self.all_iter().take(self.count() - 1)
    }

    /// Returns an iterator that contains all of the names in the original domain.
    /// The root is treated as the public suffix and therefore entirely occupies the last slot.
    ///
    /// ```
    /// # use ripgen_lib::DomainComponents;
    /// # use std::convert::TryFrom;
    /// let domain_component = DomainComponents::try_from("www.google.com")
    ///     .expect("Failed to parse.");
    ///
    /// let mut iter = domain_component
    ///     .all_iter()
    ///     .map(|elem| *elem);
    ///
    /// assert_eq!(iter.next(), Some("www"));
    /// assert_eq!(iter.next(), Some("google.com"));
    /// ```
    pub fn all_iter(&self) -> impl Iterator<Item = &&'domain str> {
        self.components.iter()
    }

    /// Returns the number of subdomain names + 1 (for root)
    pub fn count(&self) -> usize { self.components.len() }

    /// Returns a slice of the inner domain components that represent the subdomain names
    pub fn subdomains(&self) -> &[&str] {
        &self.components[.. self.count() - 1]
    }

    /// Returns the root
    ///
    /// ```
    /// # use ripgen_lib::DomainComponents;
    /// # use std::convert::TryFrom;
    /// let domain_component = DomainComponents::try_from("www.google.com")
    ///     .expect("Failed to parse.");
    ///
    /// assert_eq!(domain_component.root(), "google.com");
    /// ```
    pub fn root(&self) -> &str {
        self.components[self.components.len() - 1]
    }

    /// Returns a slice with all domain components in it. This includes the root element.
    pub fn all(&self) -> &[&str] {
        &self.components
    }
}

impl<'domain> TryFrom<&'domain str> for DomainComponents<'domain> {
    type Error = RipGenError;

    fn try_from(domain: &'domain str) -> Result<Self, Self::Error> {
        let parsed_domain_name: Name<'domain> = parse_dns_name(domain)
            .map_err(|_| RipGenError::ErrorParsingDomain(domain.to_string()))?;

        let root: &str = parsed_domain_name
            .root()
            .unwrap_or(domain);

        // we do this so we can appease lifetimes
        let root_start = domain.len() - root.len();
        let root: &'domain str = &domain[root_start..];

        let components: Vec<&'domain str> = domain
            .trim_end_matches(root)
            .split('.')
            .filter(|elem| !elem.is_empty())
            .chain(vec![root])
            .collect::<Vec<&str>>();

        let new_components = Self {
            components
        };

        Ok(new_components)
    }
}
