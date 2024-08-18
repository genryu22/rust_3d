fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<String>();

    std::thread::spawn(move || loop {
        if let Ok(res) = rx.try_recv() {
            run_command(convert_to_command(res.as_str()));
        }
    });

    loop {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");
        tx.send(buffer.trim().to_string()).unwrap();
    }
}

fn convert_to_command(raw_command: &str) -> Command {
    let args: Vec<&str> = raw_command.split(' ').collect();
    if args.len() == 0 {
        Command::None
    } else {
        match args[0] {
            "CreateClass" => Command::CreateClass,
            _ => Command::None,
        }
    }
}

fn run_command(command: Command) {
    println!("{:?}", command);
}

#[derive(Debug)]
enum Command {
    CreateClass,
    None,
}
