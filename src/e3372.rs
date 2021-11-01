use std::io::Read;

use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;

pub struct E3372 {
    _base_url: String,
    _client: reqwest::blocking::Client,
    pub _sent_sms: Vec<SMS>,
    pub _receiveid_sms: Vec<SMS>
}

impl E3372 {
    const CUSTOM_HEADER: &'static str = "__RequestVerificationToken";

    pub fn new(url: String) -> E3372 {
        let mut builder = reqwest::blocking::ClientBuilder::new();
        builder = builder.cookie_store(true);
        let new_e3372 = E3372 {
            _base_url: url.clone(),
            _client: builder.build().unwrap(),
            _sent_sms: vec![],
            _receiveid_sms: vec![]
        };
        return new_e3372;
    }

    pub fn get_sms_count(&self) -> String {
        let csrf_token = E3372::extract_csrf_token(&self.request_cookie_token());
        const CUSTOM_HEADER: &'static str = "__RequestVerificationToken";
        let mut res= self._client.get("http://192.168.8.1/api/sms/sms-count").header(CUSTOM_HEADER, csrf_token).send().unwrap();
        let mut body = String::from("");
        res.read_to_string(&mut body).unwrap();
        return body;
    }

    pub fn get_sms_list(&mut self, outbox: bool) -> String {
        let csrf_token = E3372::extract_csrf_token(&self.request_cookie_token());
        // boxtype = 1 -> recv
        // boxtype = 2 -> sent
        let mut res = self._client.post("http://192.168.8.1/api/sms/sms-list").header(E3372::CUSTOM_HEADER, csrf_token.clone()).body(format!("{}{}{}", r#"<?xml version: "1.0" encoding="UTF-8"?><request><PageIndex>1</PageIndex><ReadCount>20</ReadCount><BoxType>"#,
                                                                                                                                             if outbox {"2"}else{"1"}, r#"</BoxType><SortType>0</SortType><Ascending>0</Ascending><UnreadPreferred>0</UnreadPreferred></request>"#)).send().unwrap();
        let mut body = String::from("");
        res.read_to_string(&mut body).unwrap();
        self.fill_sms_list(&body, outbox);
        return body;
    }

    fn request_cookie_token(&self) -> String {
        let mut res = self._client.get("http://192.168.8.1/html/smsinbox.html?smssent").send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        return body;
    }

    fn extract_csrf_token(body: &str) -> String {
        let re = Regex::new(r#"<meta name="csrf_token" content="(.*)"/>"#).unwrap();
        let mut token: String = String::from("");
        for line in body.lines() {
            if re.is_match(line) {
                let tokens = re.captures(line).unwrap();
                if tokens.len() >= 2 {
                    token = String::from(&tokens[1]);
                }
            }
        }
        return String::from(token);
    }

    fn fill_sms_list(&mut self, xml_resp: &str, outbox: bool) -> () {
        let mut reader = Reader::from_str(xml_resp);
        reader.trim_text(true);
        let mut txt: String = String::new();
        let mut buf = Vec::new();
        let mut sms: SMS = SMS { _phone: "".to_string(), _message: "".to_string() };
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"Phone" => {
                            sms._phone = txt.clone();
                            txt.clear();
                        },
                        b"Content" => {
                            sms._message = txt.clone();
                            txt.clear();
                        },
                        b"Message" =>
                            match outbox {
                                true => self._sent_sms.push(sms.clone()),
                                false => self._receiveid_sms.push(sms.clone())
                            },
                        _ => ()
                    }
                },
                Ok(Event::Text(e)) => txt = e.unescape_and_decode(&reader).unwrap(),
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
    }
}

pub struct SMS {
    pub(crate) _phone: String,
    pub(crate) _message: String
}

impl Clone for SMS {
    fn clone(&self) -> Self {
        Self {
            _phone: self._phone.clone(),
            _message: self._message.clone()
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self._phone = source._phone.clone();
        self._message = source._message.clone();
    }
}