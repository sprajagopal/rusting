enum Message {
    Error { msg: String, id: u32 },
    Warning { msg: String, id: u32 },
    Success,
}

impl Message {
    fn trace(&self) -> String {
        match self {
            Message::Error { id, msg } => format!("[E{}] {}", id, msg),
            Message::Warning { id, msg } => format!("[E{}] {}", id, msg),
            Message::Success => String::from("[NOERROR] success"),
        }
    }
}

fn main() {
    let m_error = Message::Error {
        msg: String::from("Does not exist"),
        id: 1,
    };
    let m_warning = Message::Warning {
        msg: String::from("Deprecated"),
        id: 1,
    };
    let m_success = Message::Success;

    println!("{}", m_error.trace());
    println!("{}", m_warning.trace());
    println!("{}", m_success.trace());

}
