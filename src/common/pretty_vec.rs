use std::fmt;

pub struct PrettyVec<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> PrettyVec<T> {
    pub fn from_1d(vec: Vec<T>) -> Self {
        Self { data: vec![vec] }
    }

    pub fn from_2d(matrix: Vec<Vec<T>>) -> Self {
        Self { data: matrix }
    }
}

impl<T: fmt::Debug> fmt::Debug for PrettyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.is_empty() || self.data.iter().all(|row| row.is_empty()) {
            return write!(f, "[Empty Board]");
        }

        let formatted_matrix: Vec<Vec<String>> = self.data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|val| {
                        let s = format!("{:?}", val);
                        let char_count = s.chars().count();
                        
                        if char_count > 3 {
                            let truncated: String = s.chars().take(3).collect();
                            format!("{}...", truncated)
                        } else {
                            s
                        }
                    })
                    .collect()
            })
            .collect();

        let max_width = formatted_matrix
            .iter()
            .flat_map(|row| row.iter())
            .map(|s| s.chars().count())
            .max()
            .unwrap_or(0);

        let cell_border = "-".repeat(max_width + 2);

        for row in &formatted_matrix {
            for _ in 0..row.len() {
                write!(f, "+{}", cell_border)?;
            }
            writeln!(f, "+")?;

            for val in row {
                write!(f, "| {:<width$} ", val, width = max_width)?;
            }
            writeln!(f, "|")?;
        }

        if let Some(last_row) = formatted_matrix.last() {
            for _ in 0..last_row.len() {
                write!(f, "+{}", cell_border)?;
            }
            write!(f, "+")?;
        }

        Ok(())
    }
}
