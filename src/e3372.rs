use std::io::Read;
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use chrono::{Local, NaiveDate, NaiveDateTime};

pub struct E3372 {
    _base_url: String,
    _client: reqwest::blocking::Client,
    _sms_count_in_out: (u32, u32),
    pub sent_sms: Vec<SMS>,
    pub received_sms: Vec<SMS>
}

#[allow(dead_code)]
impl E3372 {
    const CUSTOM_HEADER: &'static str = "__RequestVerificationToken";

    pub fn new(url: String) -> E3372 {
        let mut builder = reqwest::blocking::ClientBuilder::new();
        builder = builder.cookie_store(true);
        let new_e3372 = E3372 {
            _base_url: url.clone(),
            _client: builder.build().unwrap(),
            _sms_count_in_out: (0, 0),
            sent_sms: vec![],
            received_sms: vec![]
        };
        return new_e3372;
    }

    pub fn fetch_all_data(mut self) -> Result<Self, ()> {
        if self.fetch_sms_count().is_err() {
            return Err(());
        }
        if self.fetch_sms_list(false).is_err() || self.fetch_sms_list(true).is_err() {
            return Err(());
        }
        return Ok(self);
    }

    fn fetch_sms_count(&mut self) -> Result<(), ()> {
        let csrf_token = E3372::extract_csrf_token(&self.request_cookie_token().unwrap()).unwrap();
        let mut res = self._client.get(format!("{}/api/sms/sms-count", self._base_url)).header(E3372::CUSTOM_HEADER, csrf_token).send().unwrap();
        if res.status() != reqwest::StatusCode::OK {
            println!("Error: {}", res.status());
            return Err(());
        }
        let mut body = String::from("");
        res.read_to_string(&mut body).unwrap();
        if body.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n<error>") {
            println!("Error: {}", body);
            return Err(());
        }
        self._sms_count_in_out = self.extract_sms_count_from_xml(&body);
        return Ok(());
    }

    fn extract_sms_count_from_xml(&self, xml: &str) -> (u32, u32) {
        let re = Regex::new(r"<LocalInbox>(\d+)</LocalInbox>").unwrap();
        let re2 = Regex::new(r"<LocalOutbox>(\d+)</LocalOutbox>").unwrap();
        let cap = re.captures(xml).unwrap();
        let cap2 = re2.captures(xml).unwrap();
        return (cap[1].parse::<u32>().unwrap(), cap2[1].parse::<u32>().unwrap());
    }

    pub fn delete_sms_list(&self, sms_list: &Vec<SMS>) -> Result<(), ()> {
        let csrf_token = E3372::extract_csrf_token(&self.request_cookie_token().unwrap()).unwrap();
        let sms_index_list: Vec<String> = sms_list.iter().map(|sms| sms.index.to_string()).collect();
        let xml_body = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><request><Index>{}</Index></request>", sms_index_list.join("</Index><Index>"));
        let res = self._client.post(format!("{}/api/sms/delete-sms", self._base_url)).header(E3372::CUSTOM_HEADER, csrf_token).body(xml_body).send().unwrap();
        match res.status().is_success() && res.text().unwrap().contains("<response>OK</response>") {
            true => {
                return Ok(());
            }
            false => {
                return Err(());
            }
        }
    }

    pub fn send_sms(&self, phone: &str, content: &str) -> Result<(), ()> {
        let csrf_token = E3372::extract_csrf_token(&self.request_cookie_token().unwrap()).unwrap();
        let xml_body = format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?><request><Index>-1</Index><Phones><Phone>{}</Phone></Phones><Sca></Sca><Content>{}</Content><Length>{}</Length><Reserved>1</Reserved><Date>{}</Date></request>", phone, content, content.len(), Local::now().format("%Y-%m-%d %H:%M:%S").to_string());
        let res = self._client.post(format!("{}/api/sms/send-sms", self._base_url)).header(E3372::CUSTOM_HEADER, csrf_token).body(xml_body).send().unwrap();
        match res.status().is_success() && res.text().unwrap().contains("<response>OK</response>") {
            true => {
                return Ok(());
            }
            false => {
                return Err(());
            }
        }
    }

    // boxtype = 1 -> recv
    // boxtype = 2 -> sent
    fn fetch_sms_list(&mut self, outbox: bool) -> Result<(), ()> {
        let mut total_sms_count: i32 = if outbox { self._sms_count_in_out.1 } else { self._sms_count_in_out.0 } as i32;
        let mut page_index = 1;
        let box_type = if outbox { 2 } else { 1 };
        while total_sms_count > 0 {
            let csrf_token = E3372::extract_csrf_token(&self.request_cookie_token().unwrap()).unwrap();
            let xml_body = format!("<?xml version: \"1.0\" encoding=\"UTF-8\"?><request><PageIndex>{}</PageIndex><ReadCount>50</ReadCount><BoxType>{}</BoxType><SortType>0</SortType><Ascending>0</Ascending><UnreadPreferred>0</UnreadPreferred></request>", page_index, box_type);
            let mut res = self._client.post(format!("{}/api/sms/sms-list", self._base_url)).header(E3372::CUSTOM_HEADER, csrf_token.clone()).body(xml_body).send().unwrap();
            let mut body = String::from("");
            if res.read_to_string(&mut body).is_err() {
                return Err(());
            }
            self.fill_sms_list(&body, outbox);
            if body.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n<error>") {
                return Err(());
            }
            total_sms_count -= 50;
            page_index += 1;
        }

        return Ok(());
    }

    fn request_cookie_token(&self) -> Result<String, ()> {
        let res = self._client.get(format!("{}/html/smsinbox.html?smssent", self._base_url)).send();
        if res.is_err() {
            return Err(());
        }
        let mut res = res.unwrap();
        let mut body = String::new();
        if res.read_to_string(&mut body).is_err() {
            return Err(());
        }
        return Ok(body);
    }

    fn extract_csrf_token(body: &str) -> Result<String, ()> {
        let re = Regex::new(r#"<meta name="csrf_token" content="(.*)"/>"#);
        if re.is_err() {
            return Err(());
        }
        let re = re.unwrap();
        let mut token: String = String::from("");
        for line in body.lines() {
            if re.is_match(line) {
                let tokens = re.captures(line);
                if tokens.is_none() {
                    return Err(());
                }
                let tokens = tokens.unwrap();
                if tokens.len() >= 2 {
                    token = String::from(&tokens[1]);
                }
            }
        }
        return Ok(String::from(token));
    }

    fn fill_sms_list(&mut self, xml_resp: &str, outbox: bool) -> () {
        let mut reader = Reader::from_str(xml_resp);
        reader.trim_text(true);
        let mut txt: String = String::new();
        let mut buf = Vec::new();
        let mut sms: SMS = SMS { phone: "".to_string(), message: "".to_string(), date: NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0), index: 0 };
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"Phone" => {
                            sms.phone = txt.clone();
                            txt.clear();
                        },
                        b"Content" => {
                            sms.message = txt.clone();
                            txt.clear();
                        },
                        b"Date" => {
                            sms.date = NaiveDateTime::parse_from_str(&txt, "%Y-%m-%d %H:%M:%S").unwrap();
                            txt.clear();
                        }
                        b"Index" => {
                            sms.index = txt.clone().parse::<usize>().unwrap();
                            txt.clear();
                        }
                        b"Message" =>
                            match outbox {
                                true => self.sent_sms.push(sms.clone()),
                                false => self.received_sms.push(sms.clone())
                            },
                        _ => ()
                    }
                },
                Ok(Event::Text(e)) => txt = e.unescape_and_decode(&reader).unwrap(),
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error decoding XML at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }
}

pub struct SMS {
    pub(crate) phone: String,
    pub(crate) message: String,
    pub(crate) date: NaiveDateTime,
    index: usize
}

impl Clone for SMS {
    fn clone(&self) -> Self {
        Self {
            phone: self.phone.clone(),
            message: self.message.clone(),
            date: self.date.clone(),
            index: self.index
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.phone = source.phone.clone();
        self.message = source.message.clone();
        self.date = source.date.clone();
        self.index = source.index;
    }
}