pub fn convert_to_military(timestamp: &str) -> String {
    let hours = get_hours(timestamp);
    let suffix = &timestamp[timestamp.len()-2..];

    if suffix == "AM" || suffix == "PM" {
        let new_hours =
            match suffix {
                "AM" => {
                    match hours {
                        12 => 0,
                        hours => hours,
                    }
                },
                "PM" => {
                    match hours {
                        12 => 12,
                        _ => hours + 12,
                    }
                },
                &_ => hours,
            };

        format!("{:02}:{}", new_hours, &timestamp[3..timestamp.len()-2])
    } else {
        timestamp.to_string()
    }
}

fn get_hours(timestamp: &str) -> u8 {
    let initial_two_str = &timestamp[0..2];
    let initial_two: u8 = initial_two_str.parse::<>().unwrap();
    initial_two
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_military() {
        assert_eq!(convert_to_military("07:05:45AM"), "07:05:45");
        assert_eq!(convert_to_military("07:05:45PM"), "19:05:45");
        assert_eq!(convert_to_military("07:05:45"), "07:05:45");
        assert_eq!(convert_to_military("12:01:00AM"), "00:01:00");
        assert_eq!(convert_to_military("12:01:00PM"), "12:01:00");
        assert_eq!(convert_to_military("12:05:45"), "12:05:45");
        assert_eq!(convert_to_military("00:01:00"), "00:01:00");
    }
}
