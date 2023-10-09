
fn sleep_in(weekday: bool, vacation: bool) -> bool {
    if !weekday || vacation {
        true
    } else {
        false
    }
}