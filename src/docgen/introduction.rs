use crate::errors::*;
use std::path::{Path, PathBuf};

pub fn parse_introduction(summary: &str) -> Result<Introduction> {
    let parser = IntroductionParser::new(introduction);
    parser.parse()
}

pub struct Introduction {
    pub title: Option<String>,
    pub prefix_sections: Vec<DocItem>,
    pub numbered_sections: Vec<DocItem>,
    pub suffix_sections: Vec<DocItem>,
}

pub struct Link {
    pub name: String,
    pub location: Option<PathBuf>,
    pub number: Option<SectionNumber>,
    pub nst_itms: Vec<DocItem>,
}

