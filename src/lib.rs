pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn sub(left: usize, right: usize) -> usize {
    left - right - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
