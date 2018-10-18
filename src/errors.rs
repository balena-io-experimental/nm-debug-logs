error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }

    errors {
    }
}

pub fn exit_code(e: &Error) -> i32 {
    match *e.kind() {
        _ => 1,
    }
}
