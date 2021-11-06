use std::env;

mod e3372;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage(&args);
        return;
    }
    let command_line_option = &args[1];

    let mut e33 = e3372::E3372::new("http://192.168.8.1".parse().unwrap());
    e33.fetch_all_data();
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
                            e33._sent_sms.iter().for_each(|sms| {
                                println!("Phone: {}", sms.phone);
                                println!("Message: {}", sms.message);
                                println!("Date: {}", sms.date);
                            });
                        },
                        "received" => {
                            e33._received_sms.iter().for_each(|sms| {
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
                    if !e33.send_sms(&*args[2], &*args[3]) {
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
            if !e33.delete_sms_list(&e33._sent_sms) || !e33.delete_sms_list(&e33._received_sms) {
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