#[cfg(test)]
fn get_parser() -> DateParser {
    DateParser::new(2018, 2, 22)
}

#[cfg(test)]
fn test_parser(slot: &str, expected_start: &str, expected_end: &str) {
    let parsed = get_parser().parse_time_slot(&slot.to_string());
    assert!(parsed.is_some(), "Parser returned None");
    let (start, end) = parsed.unwrap();
    assert_eq!(expected_start, start);
    assert_eq!(expected_end, end);
}

#[test]
fn test_am_am() {
    test_parser(&"Thursday, 8:00AM - 10:00AM",
                &"Thu, 22 Feb 2018 08:00:00 -0500",
                &"Thu, 22 Feb 2018 10:00:00 -0500",);
}

#[test]
fn test_am_pm() {
    test_parser(&"Thursday, 10:00AM - 2:00PM",
                &"Thu, 22 Feb 2018 10:00:00 -0500",
                &"Thu, 22 Feb 2018 14:00:00 -0500",);
}

#[test]
fn test_start_at_noon() {
    test_parser(&"Thursday, 12:00PM - 2:00PM",
                &"Thu, 22 Feb 2018 12:00:00 -0500",
                &"Thu, 22 Feb 2018 14:00:00 -0500",);
}

#[test]
fn test_end_at_noon() {
    test_parser(&"Thursday, 10:00AM - 12:00PM",
                &"Thu, 22 Feb 2018 10:00:00 -0500",
                &"Thu, 22 Feb 2018 12:00:00 -0500",);
}

#[test]
fn cross_day_boundary() {
    test_parser(&"Thursday, 10:00PM - 02:00AM",
                &"Thu, 22 Feb 2018 22:00:00 -0500",
                &"Fri, 23 Feb 2018 02:00:00 -0500",);
}

#[test]
fn test_start_at_midnight() {
    test_parser(&"Friday, 12:00AM - 4:00AM",
                &"Fri, 23 Feb 2018 00:00:00 -0500",
                &"Fri, 23 Feb 2018 04:00:00 -0500",);
}

#[test]
fn test_end_at_midnight() {
    test_parser(&"Thursday, 10:00PM - 12:00AM",
                &"Thu, 22 Feb 2018 22:00:00 -0500",
                &"Fri, 23 Feb 2018 00:00:00 -0500",);
}

