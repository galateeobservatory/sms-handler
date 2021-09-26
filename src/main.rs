mod e3372;

fn main() {
    let e33 = e3372::E3372::new("http://192.168.8.1".parse().unwrap());
    println!("{}", e33.get_sms_count());
}