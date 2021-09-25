use std::io::Read;
use regex::Regex;
mod e3372;

fn main() {
    /*let mut builder = reqwest::blocking::ClientBuilder::new();
    builder = builder.cookie_store(true);
    let client = builder.build().unwrap();
    //let client = reqwest::blocking::Client::new();
    let mut res = client.get("http://192.168.8.1/html/smsinbox.html?smssent").send().unwrap();

    let mut body = String::new();
    {
        res.read_to_string(&mut body).unwrap();
    }
    let csrf_token = get_csrf_token(&body);
    println!("{}", csrf_token);
    const CUSTOM_HEADER: &'static str = "__RequestVerificationToken";
    // boxtype = 1 -> recv
    // boxtype = 2 -> sent
    res = client.post("http://192.168.8.1/api/sms/sms-list").header(CUSTOM_HEADER, csrf_token.clone()).body(r#"<?xml version: "1.0" encoding="UTF-8"?><request><PageIndex>1</PageIndex><ReadCount>20</ReadCount><BoxType>2</BoxType><SortType>0</SortType><Ascending>0</Ascending><UnreadPreferred>0</UnreadPreferred></request>"#).send().unwrap();
    body = String::from("");
    res.read_to_string(&mut body).unwrap();
    println!("{}", body);
    // http://192.168.8.1/api/sms/sms-count
    res = client.post("http://192.168.8.1/api/sms/sms-count").header(CUSTOM_HEADER, csrf_token).send().unwrap();
    body = String::from("");
    res.read_to_string(&mut body).unwrap();
    println!("{}", body);*/
    let e33 = e3372::E3372::new("http://192.168.8.1".parse().unwrap());
    println!("{}", e33.get_sms_count());
}

fn get_csrf_token(body: &String) -> String {
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