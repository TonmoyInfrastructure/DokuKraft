use std::path::PathBuf; 
/* 
* Documentation : https://doc.rust-lang.org/std/path/struct.PathBuf.html
* An owned, mutable path (akin to String).
* This type provides methods like push and set_extension that mutate the path in place.
* It also implements Deref to Path, meaning that all methods on Path slices are available 
* on PathBuf values as well.
*/


pub struct DocGenerator{
    root: PathBuf,
    generate_gitign: bool,
    
}