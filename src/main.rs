use std::io::Read;
use regex::Regex;

fn main() {
    // let mut res = reqwest::blocking::get("http://192.168.8.1/api/sms/sms-list").unwrap();
    let mut res =reqwest::blocking::get("http://192.168.8.1/html/smsinbox.html?smssent").unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    //println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());
    //println!("Body:\n{}", body);
    println!("{}", get_csrf_token(&body));

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