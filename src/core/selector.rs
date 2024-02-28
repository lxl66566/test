use super::word::Word;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Selector {
    /// The name of this selector
    pub name: String,
    /// The type of this selector
    pub selector_type: SelectorType,
    /// selector, the String members become String CSS selectors.
    pub selector: Word,
    /// The format URL to fetch
    pub url: Url,
    /// one word in online page may have many paragraph, each paragraph has diffirent meanings. The `field` is to select diffirent fields.
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
