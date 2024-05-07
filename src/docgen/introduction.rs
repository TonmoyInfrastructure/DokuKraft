use crate::errors::*;
use std::path::{Path, PathBuf};

// Represents the introduction text that needs to be parsed.
pub fn parse_introduction(introduction: &str) -> Result<Introduction> {
    let parser = IntroductionParser::new(introduction); //Creates a new instance of IntroductionParser by calling its new associated function with introduction as an argument. 
    parser.parse() //implemented for IntroductionParser instances, responsible for parsing the introduction text. 
}

// structured introduction typically found in documents
pub struct Introduction {
    pub title: Option<String>, //represents the title of the introduction
    pub prefix_sections: Vec<DocItem>, //contains the sections that come before any numbered sections in the introduction
    pub numbered_sections: Vec<DocItem>, //specifically for sections that are numbered.
    pub suffix_sections: Vec<DocItem>, //contains the sections that come after any numbered sections in the introduction.
}

//represents a hyperlink or reference within a document
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub name: String, //holds the name or title of the link
    pub location: Option<PathBuf>, //represents the location that the link points to
    pub number: Option<SectionNumber>, //holds the section number associated with the link
    pub nst_itms: Vec<DocItem>, //contains nested items associated with the link
}
// provides an implementation for the Link struct
impl Link {
    // S represents a type that can be converted into a String. This allows flexibility in accepting various types as the name parameter, as long as they can be converted into a String.
    // P represents a type that can be treated as a reference to a Path. This is typically used for file system paths.
    // S = represents the name or title of the link.
    // P = represents the location (such as a file system path) that the link points to.
    pub fn new<S: Into<String>, P: AsRef<Path>>(name: S, location: P) -> Link {
        // constructs a new Link instance.
        Link {
            name: name.into(), //converts the name argument into a String and assigns it to the name field of the Link instance.
            location: Some(location.as_ref().to_path_buf()), // converts the location argument into a PathBuf
            number: None, // initializes the number field to None.
            nst_itms: Vec::new(), //initializes the nst_itms field as an empty vector.
        }
    }
}

// implements the Default trait for the Link struct, allowing instances of Link to be created with default values using the Default::default() method.
impl Default for Link {
    // defines the default() associated function for the Default trait, specifying that it returns a value of type Self, which in this case is Link.
    fn default() -> Self {
        // constructs a new Link instance with default values.
        Link {
            name: String::new(), //creates an empty String and assigns it to the name field.
            location: Some(PathBuf::new()), //creates an empty PathBuf and wraps it in Some(...) to represent an optional location.
            number: None, //initializes the number field to None.
            nested_items: Vec::new(), //initializes the nested_items field as an empty vector.
        }
    }
}