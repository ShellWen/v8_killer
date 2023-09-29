use serde::Deserialize;
use crate::source::Source;

pub(crate) trait SourceMatcherTrait {
    fn matches(&self, resource: &Source) -> bool;
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SourceMatcher {
    #[serde(rename = "resource-name-keyword")]
    ResourceNameKeywordMatcher(ResourceNameKeywordMatcher),
}

impl SourceMatcher {
    pub fn matches(&self, source: &Source) -> bool {
        match self {
            SourceMatcher::ResourceNameKeywordMatcher(matcher) => matcher.matches(source),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ResourceNameKeywordMatcher {
    pub keyword: String,
}

impl SourceMatcherTrait for ResourceNameKeywordMatcher {
    fn matches(&self, source: &Source) -> bool {
        source.resource_name.contains(&self.keyword)
    }
}
