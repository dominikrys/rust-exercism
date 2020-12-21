pub fn is_yelling(message: &str) -> bool {
    let has_letters = message.chars().any(|c| c.is_alphabetic());
    message == message.to_uppercase() && has_letters
}

pub fn reply(mut message: &str) -> &str {
    message = message.trim();

    match message {
        m if m.len() == 0 => "Fine. Be that way!",
        m if m.ends_with('?') && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if m.ends_with('?') => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever."
    }
}
