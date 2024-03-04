use super::word::Word;
use scraper::{error::SelectorErrorKind, Selector};

/// WordSelector is to transfrom a Word (constructed with String) to Selectors.
pub struct WordSelector {
    pub it: Selector,
    pub metadata: Selector,
    pub definition: Selector,
    pub example: Selector,
}

impl<'a> TryFrom<&'a Word> for WordSelector {
    type Error = SelectorErrorKind<'a>;
    fn try_from(value: &'a Word) -> Result<Self, Self::Error> {
        Ok(WordSelector {
            it: Selector::parse(&value.it)?,
            metadata: Selector::parse(&value.metadata)?,
            definition: Selector::parse(&value.definition)?,
            example: Selector::parse(&value.example)?,
        })
    }
}
