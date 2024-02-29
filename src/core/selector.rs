use super::word::Word;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Selector {
    /// The name of this selector
    pub name: String,
    /// The type of this selector
    pub selector_type: SelectorType,
    /// selector, the String members become String CSS selectors.
    pub selector: Word,
    /// The format URL to fetch
    pub url: String,
    /// select diffirent fields which may have diffirent meaning.
    pub field: Vec<String>,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum SelectorType {
    /// API unimplemented.
    Api,
    #[default]
    Css,
}

impl Selector {
    pub fn new<T1, V>(name: T1, selector: Word, url: String, field: V) -> Self
    where
        T1: Into<String>,
        V: IntoIterator,
        V::Item: Into<String>,
    {
        Selector {
            name: name.into(),
            selector_type: SelectorType::Css,
            selector,
            url,
            field: field.into_iter().map(Into::into).collect(),
        }
    }
}
