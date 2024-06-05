use die_exit::*;
use scraper::{error::SelectorErrorKind, Html};
use serde::{Deserialize, Serialize};

use super::word::{Delimiter, Word};

/// The actual struct to select html.
pub struct RealSelector {
    pub it: scraper::Selector,
    pub metadata: scraper::Selector,
    pub definition: scraper::Selector,
    pub example: scraper::Selector,
}

impl<'a> TryFrom<&'a RealSelectorString> for RealSelector {
    type Error = SelectorErrorKind<'a>;
    fn try_from(value: &'a RealSelectorString) -> Result<Self, Self::Error> {
        Ok(RealSelector {
            it: scraper::Selector::parse(&value.it)?,
            metadata: scraper::Selector::parse(&value.metadata)?,
            definition: scraper::Selector::parse(&value.definition)?,
            example: scraper::Selector::parse(&value.example)?,
        })
    }
}

/// Serializable version for RealSelector
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RealSelectorString {
    pub it: String,
    pub metadata: String,
    pub definition: String,
    pub example: String,
}

impl RealSelectorString {
    pub fn new<S1, S2, S3, S4>(it: S1, metadata: S2, definition: S3, example: S4) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        RealSelectorString {
            it: it.into(),
            metadata: metadata.into(),
            definition: definition.into(),
            example: example.into(),
        }
    }

    /// select one word.
    pub fn select_one(&self, html: &Html, word: &str, d: &Delimiter) -> Word {
        let wordselector = RealSelector::try_from(self)
            .die_with(|err| format!("Selector construction error: {}", err));
        let select = |s: &scraper::Selector, delimiter: &str| {
            html.select(s)
                .map(|dom| dom.text().collect::<Vec<&str>>().concat())
                .collect::<Vec<_>>()
                .join(delimiter)
        };
        // make Word.it not null
        let mut it = select(&wordselector.it, d.it.as_str()).trim().to_string();
        if it.is_empty() {
            it = word.to_string();
        }
        Word::new(
            // word,
            it,
            select(&wordselector.metadata, d.metadata.as_str()),
            select(&wordselector.definition, d.definition.as_str()),
            select(&wordselector.example, d.example.as_str()),
        )
    }
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Selector {
    /// The name of this selector
    pub name: String,
    /// The type of this selector
    pub selector_type: SelectorType,
    /// selector, the String members become String CSS selectors.
    pub selector: RealSelectorString,
    /// The format URL to fetch
    pub url: String,
    // select diffirent fields which may have diffirent meaning.
    // pub field: Vec<String>,
    pub delimiter: Delimiter,
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
    pub fn new<T1, T2>(
        name: T1,
        selector: RealSelectorString,
        url: T2,
        delimiter: Delimiter,
    ) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
        // V: IntoIterator,
        // V::Item: Into<String>,
    {
        Selector {
            name: name.into(),
            selector_type: SelectorType::Css,
            selector,
            url: url.into(),
            // field: field.into_iter().map(Into::into).collect(),
            delimiter,
        }
    }

    pub fn select_one(&self, html: &Html, word: &str) -> Word {
        self.selector.select_one(html, word, &self.delimiter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_one() {
        let html = Html::parse_document(
            r#"
            <div>
                <div class="it">TestWord</div>
                <div class="it">TestWord</div>
                <div class="metadata">TestMetadata</div>
                <div class="metadata">TestMetadata</div>
                <div class="definition">TestDefinition</div>
                <div class="definition">TestDefinition</div>
                <div class="example">TestExample</div>
                <div class="example">TestExample</div>
            </div>
        "#,
        );
        let delimiter = Delimiter::new("_", ", ", "; ", ". ");
        let realselectorstring =
            RealSelectorString::new(".it", ".metadata", ".definition", ".example");
        let result_word = realselectorstring.select_one(&html, "", &delimiter);
        let expected_word = Word::new(
            "TestWord_TestWord",
            "TestMetadata, TestMetadata",
            "TestDefinition; TestDefinition",
            "TestExample. TestExample",
        );
        assert_eq!(result_word.it, expected_word.it);
        assert_eq!(result_word.metadata, expected_word.metadata);
        assert_eq!(result_word.definition, expected_word.definition);
        assert_eq!(result_word.example, expected_word.example);
    }
}
