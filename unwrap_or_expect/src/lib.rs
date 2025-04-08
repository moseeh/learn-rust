pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security:: Unknown => server.unwrap(),
        Security:: Message => server.expect("ERROR: program stops"),
        Security:: Warning => server.expect("WARNING: check the server"),
        Security:: NotFound => server.expect("Not found: [{}]", server.ERROR),
        Security:: UnexpectedUrl => server.unwrap()
    }
}
