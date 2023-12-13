pub fn min_and_max_sums(arr: &[i32]) -> Option<(i64, i64)> {
    let chunk_size = 4;

    if arr.len() > chunk_size {
        let mut sorted: Vec<i64> = arr.iter().map(|&i| i.into()).collect();

        sorted.sort();

        let smallest = &sorted[0..chunk_size];
        let biggest = &sorted[sorted.len()-chunk_size..];

        Some((smallest.iter().sum(), biggest.iter().sum()))
    } else {
        None
    }
}

pub fn min_and_max_sums_string(arr: &[i32]) -> String {
    match min_and_max_sums(arr) {
        None => String::new(),
        Some((biggest_sum, smallest_sum)) => format!("{} {}", biggest_sum, smallest_sum),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_and_max_sums() {
        assert_eq!(min_and_max_sums(&[1,3]), None);
        assert_eq!(min_and_max_sums(&[1,3,5,7,9]), Some((16, 24)));
        assert_eq!(min_and_max_sums(&[1,2,3,4,5]), Some((10, 14)));
        assert_eq!(min_and_max_sums(&[256741038, 623958417, 467905213, 714532089, 938071625]), Some((2063136757, 2744467344)));
    }

    #[test]
    fn test_min_and_max_sums_string() {
        assert_eq!(min_and_max_sums_string(&[1,3]), "");
        assert_eq!(min_and_max_sums_string(&[1,3,5,7,9]), "16 24");
        assert_eq!(min_and_max_sums_string(&[1,2,3,4,5]), "10 14");
        assert_eq!(min_and_max_sums_string(&[256741038, 623958417, 467905213, 714532089, 938071625]), "2063136757 2744467344");
    }
}
