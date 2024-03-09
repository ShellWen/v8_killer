use serde::Deserialize;
use tracing::warn;

use crate::source::Source;

pub(crate) trait SourceMatcher {
    fn matches(&self, resource: &Source) -> bool;
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub(crate) enum SourceMatcherEnum {
    #[serde(rename = "resource-name-keyword")]
    ResourceNameKeyword(ResourceNameKeywordMatcher),
    #[serde(rename = "resource-name-regexp")]
    ResourceNameRegexp(ResourceNameRegexpMatcher),
}

impl SourceMatcher for SourceMatcherEnum {
    fn matches(&self, source: &Source) -> bool {
        match self {
            SourceMatcherEnum::ResourceNameKeyword(matcher) => matcher.matches(source),
            SourceMatcherEnum::ResourceNameRegexp(matcher) => matcher.matches(source),
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

#[derive(Deserialize, Debug)]
pub struct ResourceNameRegexpMatcher {
    pub regexp: String,
}

impl SourceMatcher for ResourceNameRegexpMatcher {
    fn matches(&self, source: &Source) -> bool {
        let re = regex::Regex::new(&self.regexp);
        match re {
            Ok(re) => re.is_match(&source.resource_name),
            Err(_) => {
                warn!("Invalid regexp: {}", &self.regexp);
                warn!("It will be ignored. Please check your config file");
                false
            }
        }
    }
}
