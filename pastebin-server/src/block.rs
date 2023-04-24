use crate::config::Config;
use crate::dto::SaveRecordInput;

use anyhow::Result;
use regex::RegexSet;

pub struct BlockRules {
    rules: RegexSet,
}

impl BlockRules {
    pub fn new(config: &Config) -> Result<Option<Self>> {
        let Some(regexps) = &config.security.block_rules else { return Ok(None) };

        if regexps.is_empty() {
            return Ok(None);
        }

        let rules = RegexSet::new(regexps)?;
        Ok(Some(Self { rules }))
    }

    pub fn is_match(&self, input: &SaveRecordInput) -> bool {
        self.rules.is_match(&input.title) || self.rules.is_match(&input.content)
    }
}
