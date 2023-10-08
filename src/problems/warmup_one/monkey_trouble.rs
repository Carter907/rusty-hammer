fn monkey_trouble(a_smile: bool, b_smile: bool) -> bool {
    !(a_smile ^ b_smile)
}

#[cfg(test)]
mod test {
    use crate::problems::warmup_one::monkey_trouble::monkey_trouble;

    #[test]
    fn test_monkey_trouble() {
        assert!(monkey_trouble(true, true));
        assert!(monkey_trouble(false, false));
        assert!(monkey_trouble(false, true), false);
        assert!(monkey_trouble(true, false), false);
    }
}