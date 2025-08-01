
pub enum MsgTemplate {
    Welcome(u64),
    Bye,
}

impl MsgTemplate {
    pub fn format(&self) -> String {
        match self {
            MsgTemplate::Welcome(ts) => format!("Welcome to Airdrop! Your login timestamp is: {}", ts),
            MsgTemplate::Bye => format!("Bye! See you..."),
        }
    }
}

