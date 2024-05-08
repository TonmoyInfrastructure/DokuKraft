use crate::errors::*;
use std::path::{Path, PathBuf};

// Represents the introduction text that needs to be parsed.
pub fn parse_introduction(introduction: &str) -> Result<Introduction> {
    let parser = IntroductionParser::new(introduction); //Creates a new instance of IntroductionParser by calling its new associated function with introduction as an argument. 
    parser.parse() //implemented for IntroductionParser instances, responsible for parsing the introduction text. 
}

// structured introduction typically found in documents
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocItem {
    Link(Link), //represents a link within the document
    Separator, //represents a separator within the document
    PartTitle(String), //represents a title for a part or section within the document
}

impl DocItem {
    // returns a mutable reference to a Link if self is a DocItem::Link, otherwise it returns None.
    fn conf_link_mut(&mut self) -> Option<&mut Link> {
        //starts a pattern match on self
        match *self {
            DocItem::Link(ref mut l) => Some(l), //extracts a mutable reference to the Link contained within self (represented by ref mut l) and returns it wrapped in Some.
            _ => None, //default branch of the match. If self is not a DocItem::Link, it returns None.
        }
    }
}

//implements the From trait for converting from a Link into a DocItem
//implementing the From trait for converting from a Link into a DocItem.
impl From<Link> for DocItem {
    //defines the implementation of the from associated function for the From trait. It takes a Link (named other) as input and returns a DocItem.
    fn from(other: Link) -> DocItem {
        DocItem::Link(other) //creates and returns a DocItem::Link variant, wrapping the provided Link instance (other).
    }
}

//struct represents a parser for parsing introductions from Markdown source text
struct IntroductionParser<'a> {
    src: &'a str, //holds a reference (&'a str) to the Markdown source text that is being parsed
    stream: pulldown_cmark::OffsetIter<'a, DefaultBrokenLinkCallback>, //parameterized with the same lifetime 'a, ensuring that the events iterator doesn't outlive the Markdown source text.
    offset: usize, // represents the current position of the parser within the Markdown source text
    back: Option<Event<'a>>, //an optional event that allows the parser to store an event temporarily
}

//simplify the process of collecting Markdown events from a stream until a certain delimiter is encountered.
macro_rules! collect_events { //macro named collect_events.
    //expects an expression $stream representing the event stream and a pattern $delimiter representing the starting delimiter. 
    ($stream:expr,start $delimiter:pat) => {
        //recursively calls collect_events! with the given stream and the pattern Event::Start($delimiter).
        collect_events!($stream, Event::Start($delimiter))
    };
    //expects to encounter an event that matches Event::End($delimiter).
    ($stream:expr,end $delimiter:pat) => {
        collect_events!($stream, Event::End($delimiter)) //recursively calls collect_events! with the given stream and the pattern Event::End($delimiter).
    };
    //default pattern. It expects any other pattern.
    ($stream:expr, $delimiter:pat) => {{
        let mut events = Vec::new(); //initializes an empty vector events to store collected events.
        //continuously fetches the next event from the stream and maps it to get only the event part, discarding the range.
        loop {
            let event = $stream.next().map(|(ev, _range)| ev);
            trace!("Next event: {:?}", event); //logs the next event for debugging purposes.
            //If the next event matches the delimiter, the loop breaks, indicating that all events have been collected until this delimiter.
            match event {
                Some($delimiter) => break,
                Some(other) => events.push(other), //If the event is not a delimiter, it's pushed into the events vector.
                //If there are no more events in the stream, it logs a message and breaks the loop.
                None => {
                    debug!(
                        "Reached end of stream without finding the closing pattern, {}",
                        stringify!($delimiter)
                    );
                    break;
                }
            }
        }

        events //returns the collected events vector.
    }};
}

