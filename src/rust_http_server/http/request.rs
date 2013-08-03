use std::ptr::to_unsafe_ptr;
use std::str;
use std::hashmap::HashMap;
use http::parser::{Parser, ParserCallbacks};


struct Request {
    url: ~str,
    headers: ~HashMap<~str, ~str>,
    body: ~str,
    callbacks: Option<ParserCallbacks>,
    current_header: ~str,
    parse_finished: bool,
}


impl Request {
    pub fn new() -> Request {
        Request {
            url: ~"",
            headers: ~HashMap::new(),
            body: ~"",
            callbacks: None,
            parse_finished: false,
            current_header: ~"",
        }
    }

    pub fn parse(&mut self, parser: &mut Parser, data: &[u8]) -> bool{
        let unsafe_self = to_unsafe_ptr(&self);

        let callbacks = ParserCallbacks {
            on_message_begin:       || unsafe { (*unsafe_self).on_message_begin() },
            on_url:                 |data| unsafe { (*unsafe_self).on_url(data) },
            on_header_field:        |data| unsafe { (*unsafe_self).on_header_field(data) },
            on_header_value:        |data| unsafe { (*unsafe_self).on_header_value(data) },
            on_headers_complete:    || unsafe { (*unsafe_self).on_headers_complete() },
            on_body:                |data| unsafe { (*unsafe_self).on_body(data) },
            on_message_complete:    || unsafe { (*unsafe_self).on_message_complete() },
        };

        self.callbacks = Some(callbacks);

        let callbacks = &self.callbacks.unwrap();
        let parsed_bytes = parser.execute(data, callbacks);

        if parsed_bytes != data.len() {
            fail!("parser failure: %u %u %u", parsed_bytes, data.len(), parser.status_code());
        }

        self.callbacks = None;

        true
    }

    pub fn on_message_begin(&self) -> bool {
        true
    }
    pub fn on_url(&mut self, data: ~[u8]) -> bool {
        let data_str = str::from_bytes(data);
        self.url = data_str;

        true
    }
    pub fn on_header_field(&mut self, data: ~[u8]) -> bool {
        let data_str = str::from_bytes(data);
        self.current_header = data_str;

        true
    }
    pub fn on_header_value(&mut self, data: ~[u8]) -> bool {
        let data_str = str::from_bytes(data);
        self.headers.insert(self.current_header.clone(), data_str);

        true
    }
    pub fn on_headers_complete(&self) -> bool {
        true
    }
    pub fn on_body(&mut self, data: ~[u8]) -> bool {
        let data_str = str::from_bytes(data);
        self.body = data_str;

        true
    }
    pub fn on_message_complete(&mut self) -> bool {
        self.parse_finished = true;
        true
    }
}


