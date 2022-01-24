use crate::domain::DomainComponents;


pub(crate) fn extract_words<'iter, 'domain>(
    domain_components: impl Iterator<Item = &'iter DomainComponents<'domain>> + 'iter,
    filter_function: &'iter impl Fn(&&str) -> bool,
) -> impl Iterator<Item = &'domain str> + 'iter
    where
        'domain: 'iter
{
    domain_components
        .flat_map(move |domain| {
            let augments = domain
                .subdomains_iter()
                .flat_map(|elem| elem.split('-'));

            domain
                .subdomains_iter().copied()
                .chain(augments)
                .filter(filter_function)
        })
}
