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
        Command::none()
    } else {
        match args[0] {
            "CreateClass" => Command::new(
                CommandType::CreateClass,
                args.iter().skip(1).map(|s| s.to_string()).collect(),
            ),
            _ => Command::none(),
        }
    }
}

fn run_command(command: Command) {
    match command.command_type {
        CommandType::CreateClass => {
            println!("Creating class with args: {:?}", command.args);
        }
        CommandType::None => {}
    }
}

struct Command {
    command_type: CommandType,
    args: Vec<String>,
}

impl Command {
    fn new(command_type: CommandType, args: Vec<String>) -> Self {
        Self { command_type, args }
    }

    fn none() -> Self {
        Self {
            command_type: CommandType::None,
            args: Vec::new(),
        }
    }
}

#[derive(Debug)]
enum CommandType {
    CreateClass,
    None,
}
