pub fn zig_zag_array(arr: &[i32]) -> Vec<i32> {
    let mut sorted: Vec<i32> = arr.to_owned();
    sorted.sort();

    let last = sorted[arr.len() - 1];

    let to_take = arr.len() / 2;

    let first_section = sorted.iter().take(to_take).collect::<Vec<_>>();
    // let first_section = first_section_iter.collect::<Vec<_>>();

    // let first_section = sorted.iter().step_by(2).take(to_take).collect::<Vec<_>>();

    let mut second_section = sorted.iter().skip(to_take).take(to_take).collect::<Vec<_>>();
        // first_section_iter.clone().take(to_take).collect::<Vec<_>>();
    second_section.reverse();

    let mut result = first_section;

    result.append(&mut vec![&last]);
    result.append(&mut second_section);

    result.iter().map(|&i| i.to_owned()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zig_zag_array() {
        assert_eq!(zig_zag_array(&[2,3,5,1,4]), vec![1,2,5,4,3]);
    }
}
