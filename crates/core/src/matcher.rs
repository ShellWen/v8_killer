use serde::Deserialize;

use crate::source::Source;

pub(crate) trait SourceMatcher {
    fn matches(&self, resource: &Source) -> bool;
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub(crate) enum SourceMatcherEnum {
    #[serde(rename = "resource-name-keyword")]
    ResourceNameKeywordMatcher(ResourceNameKeywordMatcher),
}

impl SourceMatcher for SourceMatcherEnum {
    fn matches(&self, source: &Source) -> bool {
        match self {
            SourceMatcherEnum::ResourceNameKeywordMatcher(matcher) => matcher.matches(source),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ResourceNameKeywordMatcher {
    pub keyword: String,
}

impl SourceMatcher for ResourceNameKeywordMatcher {
    fn matches(&self, source: &Source) -> bool {
        source.resource_name.contains(&self.keyword)
    }
}
