pub fn add(left: u8, right: u8) -> u8 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_out() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
