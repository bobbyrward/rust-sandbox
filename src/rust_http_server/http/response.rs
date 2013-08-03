use std::hashmap::HashMap;


pub struct Response {
    status_code: uint,
    headers: ~HashMap<~str, ~str>,
    body: ~[u8],
}


impl Response {
    pub fn new() -> Response {
        Response {
            status_code: 0,
            headers: ~HashMap::new(),
            body: ~[],
        }
    }

    pub fn set_status(&mut self, status: uint) {
        self.status_code = status;
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_owned(), value.to_owned());
    }

    pub fn set_body_str(&mut self, body: &str) {
        self.body = body.as_bytes().to_owned();
    }

    pub fn set_body_bytes(&mut self, body: ~[u8]) {
        self.body = body;
    }

    pub fn to_bytes(&mut self) -> ~[u8] {
        let mut output: ~[u8] = ~[];

        let push_line = |line: &str| {
            output.push_all(line.as_bytes());
        };

        push_line(fmt!("HTTP/1.1 %u %s\r\n", self.status_code, "OK"));

        self.headers.insert(~"Content-Length", self.body.len().to_str());

        foreach (k, v) in self.headers.iter() {
            push_line(fmt!("%s: %s\r\n", *k, *v));
        }

        push_line("\r\n");

        output.push_all(self.body);

        output
    }
}
