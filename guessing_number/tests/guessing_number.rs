use std::io::BufReader;

#[test]
fn test_main() {
    let secret = 2;
    let writer = Box::new(Vec::new());
    let reader: &[u8] = &['2' as u8; 1];
    let buf_reader = Box::new(BufReader::new(reader));
    let mut config = guessing_game::Config::new(secret, Some(buf_reader), Some(writer));

    config.run().expect("Should parser correctly")
}
