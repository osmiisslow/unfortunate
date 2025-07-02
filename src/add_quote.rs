use std::io::Read;
use curl::easy::{Easy, List};
use whoami::fallible::*;
pub fn add_quote(quote: &String) {
    let user = username().unwrap();
    let host = hostname().unwrap();
    let format = format!("{user}@{host}: \"{quote}\"");
    let mut data = format.as_bytes();
    
    // set content type to plain text
    let mut list = List::new();
    list.append("Content-Type: text/plain").unwrap();


    let mut easy = Easy::new();
    easy.url("localhost:8080").unwrap(); // TODO: For the love of god change this after we're finished with server work
    easy.http_headers(list).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.read_function(|buf| {
        Ok(data.read(buf).unwrap_or(0))
    }).unwrap();
    transfer.perform().expect("can't add quote! Maybe server is down?");
}