use std::io::IoResult;
use std::collections::hashmap::HashMap;
use std::str::from_utf8;

use super::{get_file_reader, get_file_writer};

macro_rules! parse_word(
    ($iter:ident, $word:ident, $breaker:pat, $next:expr) => (
        // Loop to parse a word out of a line
        match $iter.next() {
            Some(Ok($breaker)) => break,
            Some(Ok(c)) => $word.push(c),
            Some(Err(e)) => return Err(e),
            None => $next
        }
    );
)

// Generate response/mimegen.rs
pub fn generate(list: Path, module: Path) -> IoResult<()> {
    let mut reader = get_file_reader(list);
    let mut writer = get_file_writer(module);

    try!(writer.write(
b"// This automatically generated file is included in response/mimes.rs.

use http::headers::content_type::MediaType;

pub fn get_generated_content_type(ext: &str) -> Option<MediaType> {
    match ext {
"   ));

    /* Generated snippets will look like:
    "json" => Some(MediaType {
        type_: "application".to_str(),
        subtype: "json".to_str(),
        parameters: vec![]
    }),
    */

    let mut byter = reader.bytes();
    // avoid duplicates
    let mut seen = HashMap::new();
    'read: loop {
        let mut ext = vec![];
        let mut type_ = vec![];
        let mut subtype = vec![];
        
        loop { parse_word!(byter, ext, b' ', break 'read); }
        loop { parse_word!(byter, type_, b' ', break 'read); }
        loop{ parse_word!(byter, subtype, b'\n', break 'read); }

        if !seen.contains_key(&ext) {

            try!(write!(writer,
"    \"{}\" => Some(MediaType {{
        type_: \"{}\".to_str(),
        subtype: \"{}\".to_str(),
        parameters: vec![]
    }}),\n", from_utf8(ext.as_slice()).unwrap(),
             from_utf8(type_.as_slice()).unwrap(),
             from_utf8(subtype.as_slice()).unwrap()));

            seen.insert(ext, true);
        }
    }

    writer.write(b"        _ => None\n    }\n}\n")
}
