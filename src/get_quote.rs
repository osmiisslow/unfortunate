use curl::easy::Easy;

pub fn get_quote() {
    let mut quote: Vec<u8> = Vec::new();
    let mut easy = Easy::new();
    easy.url("localhost:8080").unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        quote.extend_from_slice(data);
        let output = String::from_utf8_lossy(&quote).to_string();
        println!("{output}");
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}