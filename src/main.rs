mod e3372;
use quick_xml::Reader;
use quick_xml::events::Event;

fn main() {
    let mut e33 = e3372::E3372::new("http://192.168.8.1".parse().unwrap());
    //println!("{}", e33.get_sms_count());
    //println!("{}", e33.get_sms_list(true));
    e33.get_sms_list(true);
    println!("{}", e33._sent_sms.get(0).unwrap()._phone);
    for sms in e33._sent_sms.iter() {
        println!("Phone: {}", sms._phone);
        println!("Message: {}", sms._message);
    }

    //let xml = e33.get_sms_count();
}