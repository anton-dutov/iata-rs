pub fn split_passenger_name(input: &str) -> (String, Option<String>) {
    let last_start_idx = 0;
    let mut last_end_idx = 0;
    let mut first_start_idx = 0;
    let mut first_end_idx = 0;
    let mut have_first = false;

    // input is ASCII so we can do byte-wise indexing safely
    for (idx, c) in input.char_indices() {
        // If haven't consumed last name yet
        if !have_first && c == '/' {
            last_end_idx = idx;
            have_first = true;
            first_start_idx = idx + 1;
            first_end_idx = input.len();
        }
    }

    // if there is no first name, surname occupies whe whole name field
    if !have_first {
        last_end_idx = input.len();
    }

    // extract names
    let last = input[last_start_idx..last_end_idx].trim_end().to_string();
    let first = if have_first {
        let first = input[first_start_idx..first_end_idx].trim_end();
        Some(first.to_string())
    } else {
        None
    };

    (last, first)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_name(
        last: String,
        first: Option<String>,
        right_last: &str,
        right_first: Option<&str>,
    ) {
        assert_eq!(&last, right_last);
        assert_eq!(first, right_first.map(String::from));
    }

    #[test]
    fn test_split_passenger_name() {
        let names = &[
            "BRUNER/ROMAN MR     ",
            "JOHN/SMITH JORDAN   ",
            "VERYLONGESTLASTNAMED",
            "JOHN/SMITH          ",
            "BRUNER ROMAN MR/    ",
        ];

        let answers = &[
            ("BRUNER", Some("ROMAN MR")),
            ("JOHN", Some("SMITH JORDAN")),
            ("VERYLONGESTLASTNAMED", None),
            ("JOHN", Some("SMITH")),
            ("BRUNER ROMAN MR", Some("")),
        ];

        for i in 0..names.len() {
            let (last, first) = split_passenger_name(names[i]);
            let (right_last, right_first) = answers[i];
            check_name(last, first, right_last, right_first)
        }
    }
}
