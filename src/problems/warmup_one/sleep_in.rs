
fn sleep_in(weekday: bool, vacation: bool) -> bool {
    if !weekday || vacation {
        true
    } else {
        false
    }
}
#[cfg(test)]
mod test {
    use crate::problems::warmup_one::sleep_in::sleep_in;

    #[test]
    fn test_sleep_in() {
        assert!(sleep_in(false, false));
        assert!(sleep_in(false, true));
        assert!(sleep_in(true, true));
        assert!(sleep_in(true, false), false);
    }
}