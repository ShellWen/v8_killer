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
    InsertBeforeProcessor(InsertBeforeProcessor),
    #[serde(rename = "insert-after")]
    InsertAfterProcessor(InsertAfterProcessor),
}

impl SourceProcessor {
    pub fn process<'a>(&self, source: &'a mut Source) -> Result<&'a mut Source, Error> {
        match self {
            SourceProcessor::InsertBeforeProcessor(processor) => processor.process(source),
            SourceProcessor::InsertAfterProcessor(processor) => processor.process(source),
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
