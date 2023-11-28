use std::fmt::Error;

use serde::Deserialize;

use crate::source::Source;

pub(crate) trait SourceProcessorTrait {
    fn process<'a>(&self, source: &'a mut Source) -> Result<&'a mut Source, Error>;
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SourceProcessor {
    #[serde(rename = "insert-before")]
    InsertBefore(InsertBeforeProcessor),
    #[serde(rename = "insert-after")]
    InsertAfter(InsertAfterProcessor),
    #[serde(rename = "replace")]
    Replace(ReplaceProcessor),
    #[serde(rename = "replace-regexp")]
    ReplaceRegexp(ReplaceRegexpProcessor),
}

impl SourceProcessor {
    pub fn process<'a>(&self, source: &'a mut Source) -> Result<&'a mut Source, Error> {
        match self {
            SourceProcessor::InsertBefore(processor) => processor.process(source),
            SourceProcessor::InsertAfter(processor) => processor.process(source),
            SourceProcessor::Replace(processor) => processor.process(source),
            SourceProcessor::ReplaceRegexp(processor) => processor.process(source),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct InsertBeforeProcessor {
    content: String,
}

impl SourceProcessorTrait for InsertBeforeProcessor {
    fn process<'a>(&self, source: &'a mut Source) -> Result<&'a mut Source, Error> {
        source.source_string = format!("{}{}", self.content, source.source_string);
        Ok(source)
    }
}

#[derive(Deserialize, Debug)]
pub struct InsertAfterProcessor {
    content: String,
}

impl SourceProcessorTrait for InsertAfterProcessor {
    fn process<'a>(&self, source: &'a mut Source) -> Result<&'a mut Source, Error> {
        source.source_string = format!("{}{}", source.source_string, self.content);
        Ok(source)
    }
}

#[derive(Deserialize, Debug)]
pub struct ReplaceProcessor {
    from: String,
    to: String,
}

impl SourceProcessorTrait for ReplaceProcessor {
    fn process<'a>(&self, source: &'a mut Source) -> Result<&'a mut Source, Error> {
        source.source_string = source.source_string.replace(&self.from, &self.to);
        Ok(source)
    }
}

#[derive(Deserialize, Debug)]
pub struct ReplaceRegexpProcessor {
    regexp: String,
    to: String,
}

impl SourceProcessorTrait for ReplaceRegexpProcessor {
    fn process<'a>(&self, source: &'a mut Source) -> Result<&'a mut Source, Error> {
        let re = regex::Regex::new(&self.regexp).unwrap();
        source.source_string = re.replace_all(&source.source_string, &self.to).to_string();
        Ok(source)
    }
}
