#[macro_export]
macro_rules! input {
    () => {{
        let mut tmp: String = Default::default();
        std::io::stdin().read_line(&mut tmp).expect("failed to read input");
        tmp
    }};

    ( $a:expr ) => {{
        use std::io::Write;
        print!("{}", $a);
        std::io::stdout().flush().expect("failed to flush");
        input!()
    }};
}
