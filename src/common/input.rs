#[macro_export]
macro_rules! input {
    () => {{
        let mut out: String = Default::default();
        std::io::stdin().read_line(&mut out).expect("failed to read input");
        out
    }};

    ( $prompt:expr ) => {{
        use std::io::Write;
        print!("{}", $prompt);
        std::io::stdout().flush().expect("failed to flush");
        input!()
    }};

    ( $prompt:expr, $type:ty ) => {{
        use std::io::Write;
        print!("{}", $prompt);
        std::io::stdout().flush().expect("failed to flush");

        let mut out: String = Default::default();
        std::io::stdin().read_line(&mut out).expect("failed to read input");
        out.trim().parse::<$type>().expect("type mismatch")
    }}
}
