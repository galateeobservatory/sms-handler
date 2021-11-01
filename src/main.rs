mod e3372;
use quick_xml::Reader;
use quick_xml::events::Event;

fn main() {
    let mut e33 = e3372::E3372::new("http://192.168.8.1".parse().unwrap());
    //println!("{}", e33.get_sms_count());
    //println!("{}", e33.get_sms_list(true));
    e33.get_sms_list(true);
    e33.get_sms_list(false);
    for sms in e33._sent_sms.iter() {
        println!("Phone: {}", sms._phone);
        println!("Message: {}", sms._message);
    }
    for sms in e33._receiveid_sms.iter() {
        println!("Phone: {}", sms._phone);
        println!("Message: {}", sms._message);
    }
}