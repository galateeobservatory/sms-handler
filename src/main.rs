use std::env;

mod e3372;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args);
        return;
    }
    let command_line_option = &args[1];

    let e33 = e3372::E3372::new("http://192.168.8.1".parse().unwrap()).fetch_all_data().unwrap();
    match &command_line_option[..] {
        "--list" | "--send" => {
            if args.len() < 3 {
                print_usage(&args);
                return;
            }
            let command_line_option2 = &args[2];
            match &command_line_option[..] {

                "--list" => {
                    match &command_line_option2[..] {
                        "sent" => {
                            e33.sent_sms.iter().for_each(|sms| {
                                println!("Phone: {}", sms.phone);
                                println!("Message: {}", sms.message);
                                println!("Date: {}", sms.date);
                            });
                        },
                        "received" => {
                            e33.received_sms.iter().for_each(|sms| {
                                println!("Phone: {}", sms.phone);
                                println!("Message: {}", sms.message);
                                println!("Date: {}", sms.date);
                            });
                        }
                        _ => {
                            print_usage(&args);
                            return;
                        }
                    }
                }
                "--send" => {
                    if args.len() < 4 {
                        print_usage(&args);
                        return;
                    }
                    if e33.send_sms(&*args[2], &*args[3]).is_err() {
                        eprintln!("Error sending sms");
                    }
                    println!("SMS sent");
                }
                _ => {
                    print_usage(&args);
                }
            }
        }
        "--clean" => {
            if e33.delete_sms_list(&e33.sent_sms).is_err() || e33.delete_sms_list(&e33.received_sms).is_err() {
                eprintln!("Failed to delete SMSs");
                return;
            }
            println!("SMSs deleted");
        }
        _ => {
            print_usage(&args);
            return;
        }
    }
}

fn print_usage(args: &Vec<String>) {
    eprintln!("Usage: {} {{--list sent|received}}|{{--send phone_number \"message\"}}|--clean", args[0]);
}