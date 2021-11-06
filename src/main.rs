mod e3372;

fn main() {
    let mut e33 = e3372::E3372::new("http://192.168.8.1".parse().unwrap());
    println!("{:?}", e33.get_sms_count());
    //println!("{}", e33.get_sms_list(true));
    e33.get_sms_list(true);
    println!("{}", e33.get_sms_list(false));
    println!("{}kkk", e33._sent_sms.len());
    /*for sms in e33._sent_sms.iter().chain(e33._received_sms.iter()) {
        println!("Phone: {}", sms.phone);
        println!("Message: {}", sms.message);
        println!("Date: {}", sms.date);
    }*/
    /*e33._sent_sms[0..3].to_vec().iter().for_each(|sms| {
        println!("Phone: {}", sms.phone);
        println!("Message: {}", sms.message);
        println!("Date: {}", sms.date);
    });*/

    //println!("{}", e33.delete_sms_list(&e33._sent_sms));
    println!("{}", e33.send_sms("**********", "Hello from Rust !!"));
}