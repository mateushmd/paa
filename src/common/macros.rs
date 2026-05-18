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
    
    ( $type:ty ) => {{
        use std::io::Write;
        print!("{} value: ", std::any::type_name::<$type>());
        std::io::stdout().flush.expect("failed to flush");
        
        let mut out: String = Default::default();
        std::io::stdin().read_line(&mut out).expect("failed to read input");
        out
            .trim()
            .parse::<$type>()
            .unwrap_or_else(|_| panic!(
                "type mismatch: expected {}", 
                std::any::type_name::<$type>()
            ))
    }};

    ( $prompt:expr, $type:ty ) => {{
        use std::io::Write;
        print!("{}", $prompt);
        std::io::stdout().flush().expect("failed to flush");

        let mut out: String = Default::default();
        std::io::stdin().read_line(&mut out).expect("failed to read input");
        out
            .trim()
            .parse::<$type>()
            .unwrap_or_else(|_| panic!(
                "type mismatch: expected {}", 
                std::any::type_name::<$type>()
            ))
    }}
}

#[macro_export]
macro_rules! input_vec {
    ( $type:ty ) => {{
        use std::io::Write;
        print!("array of {}: ", std::any::type_name::<$type>());
        std::io::stdout().flush().expect("failed to flush");

        let mut out: String = Default::default();
        std::io::stdin().read_line(&mut out).expect("failed to read input");
        out
            .split(",")
            .map(|s| s
                .trim()
                .parse::<$type>()
                .unwrap_or_else(|_| panic!(
                    "type mismatch: '{}' should be of type {}", 
                    s, std::any::type_name::<$type>())))
            .collect::<Vec<$type>>()
    }}
}
