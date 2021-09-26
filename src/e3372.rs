use regex::Regex;
//use reqwest::ClientBuilder;
use std::io::Read;

pub(crate) struct E3372 {
    _base_url: String,
    _client: reqwest::blocking::Client
}

impl E3372 {
    const CUSTOM_HEADER: &'static str = "__RequestVerificationToken";

    pub fn new(url: String) -> E3372 {
        let mut builder = reqwest::blocking::ClientBuilder::new();
        builder = builder.cookie_store(true);
        let new_e3372 = E3372 {
            _base_url: url.clone(),
            _client: builder.build().unwrap(),
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

    pub fn get_sms_list(&self) -> String {
        let csrf_token = E3372::extract_csrf_token(&self.request_cookie_token());
        // boxtype = 1 -> recv
        // boxtype = 2 -> sent
        let mut res = self._client.post("http://192.168.8.1/api/sms/sms-list").header(E3372::CUSTOM_HEADER, csrf_token.clone()).body(r#"<?xml version: "1.0" encoding="UTF-8"?><request><PageIndex>1</PageIndex><ReadCount>20</ReadCount><BoxType>2</BoxType><SortType>0</SortType><Ascending>0</Ascending><UnreadPreferred>0</UnreadPreferred></request>"#).send().unwrap();
        let mut body = String::from("");
        res.read_to_string(&mut body).unwrap();
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
}