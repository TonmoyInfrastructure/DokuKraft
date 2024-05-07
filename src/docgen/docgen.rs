use with_context::*;
/* 
* Documentation : https://docs.rs/with-context/latest/with_context/
* with-context is a set of macros to allow for easy singleton initialization & usage.
* If a singleton(context) has not been initialized, it will panic in runtime only in debug mode. 
* Release mode disables safety catches so test thoroughly.
*/

use super::introduction::{parse_introduction, Introduction};

pub fn init_docgen<P:AsRef<Path>>(src_dir: P, cfg: &BuildConfig) -> Result<Docgen> {
    let src_dir = src_dir.as_ref(); // Converts src_dir into a reference to a path, ensuring that it's usable as a path throughout the function.
    let intro_md = src_dir.join("INTRODUCTION.md"); // Constructs a path to an "INTRODUCTION.md" file within the src_dir directory.
    let mut intro_content = String::new(); // Creates a mutable string named intro_content to store the content of the introduction file.
    File::open(&intro_content) // Opens the "INTRODUCTION.md" file and reads its content into intro_content.
        .with_context(|| format!("Couldn't open INTRODUCTION.md in {:?} directory", src_dir))?
        .read_to_string(&mut intro_content)?;
    let introduction = parse_introduction(&intro_content)
        .with_context(|| format!("Introduction parsing failed for file={:?}", intro_content))?;
    if cfg.create_missing {
        create_missing(src_dir, &summary).with_context(|| "Unable to create missing introduction")?;
    }
    load_docgen_from_disk(&introduction, src_dir); // Passing the parsed introduction and src_dir as arguments.
}

fn create_missing(src_dir: &Path, summary: &Summary) -> Result<()> {

}

pub struct Docgen {
    pub sections: Vec<DocItem>
}

impl Docgen {
    pub fn new() -> Self {
        Default::default() // Create a default instance of Docgen, leveraging Rust's Default trait. 
    }
    pub fn iter(&self) -> DocItems<'_> {
        DocItems {
            items: self.sections.iter().collect(), // Returns an iterator over the DocItems within the Docgen struct.
        }
    }
    pub fn for_each_mut<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut DocItem),
    {
        for_each_mut(&mut func, &mut self.sections);
    }
}

pub fn for_each_mut<'a, F, I>(func: &mut F, items: I)
where
    F: FnMut(&mut DocItem),
    I: IntoIterator<Item = &'a mut DocItem>,
{
    for item in items {
        if let DocItem::Chapter(ch) = item {
            for_each_mut(func, &mut ch.sub_items);
        }
        func(item);
    }
}

