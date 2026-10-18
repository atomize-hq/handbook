use std::{io::{Read,Write},time::Duration};
fn main(){
    let mut input=String::new();std::io::stdin().read_to_string(&mut input).unwrap();
    if input.contains("partial-json") {
        std::io::stdout().write_all(b"{\"status\":").unwrap();std::io::stdout().flush().unwrap();
        std::fs::write(std::env::var("CAPTURE_READY").unwrap(),b"ready").unwrap();
        std::thread::sleep(Duration::from_secs(5));
    } else if input.contains("invalid-utf8") {
        std::io::stdout().write_all(&[0xff,0xc3]).unwrap();std::io::stdout().flush().unwrap();
    } else if !input.contains("empty-success") {
        println!("{}",r#"{"status":"ok","result":null}"#);
    }
}
