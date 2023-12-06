use std::ops::Range;
use d05::{DirtMap, DirtTransform, parse_transform};

#[test]
fn transform_test() {
    let transform = parse_transform("50 98 2");

    // contains 98 and 99
    assert!(transform.contains(&98));
    assert!(transform.contains(&99));

    // does not contain 97 or 100
    assert_eq!(transform.contains(&100), false);
    assert_eq!(transform.contains(&97), false);

    // transforms 98 to 50
    assert_eq!(transform.transform(&98), 50);
}

#[test]
fn completely_before_test() {
    let input_range = Range {start: 50, end: 60};
    let map = DirtMap {
        name: String::from("one"),
        transforms: vec![
            DirtTransform {
                range: Range {start: 100, end: 150},
                transform: 100
            },
            DirtTransform {
                range: Range {start: 200, end: 250},
                transform: 1000
            }
        ]
    };
    let transformed_ranges = map.to_output_ranges(&input_range);

    assert_eq!(transformed_ranges, vec![input_range]);
}

#[test]
fn completely_after_test() {
    let input_range = Range {start: 500, end: 600};
    let map = DirtMap {
        name: String::from("one"),
        transforms: vec![
            DirtTransform {
                range: Range {start: 100, end: 150},
                transform: 100
            },
            DirtTransform {
                range: Range {start: 200, end: 250},
                transform: 1000
            }
        ]
    };
    let transformed_ranges = map.to_output_ranges(&input_range);

    assert_eq!(transformed_ranges, vec![input_range]);
}

#[test]
fn clean_miss_test() {
    let input_range = Range {start: 150, end: 160};
    let map = DirtMap {
        name: String::from("one"),
        transforms: vec![
            DirtTransform {
                range: Range {start: 100, end: 150},
                transform: 100
            },
            DirtTransform {
                range: Range {start: 200, end: 250},
                transform: 1000
            }
        ]
    };
    let transformed_ranges = map.to_output_ranges(&input_range);

    assert_eq!(transformed_ranges, vec![input_range]);
}

#[test]
fn before_to_contained() {
    let input_range = Range {start: 50, end: 125};
    let map = DirtMap {
        name: String::from("one"),
        transforms: vec![
            DirtTransform {
                range: Range {start: 100, end: 150},
                transform: 100
            },
            DirtTransform {
                range: Range {start: 200, end: 250},
                transform: 1000
            }
        ]
    };
    let transformed_ranges = map.to_output_ranges(&input_range);

    assert_eq!(transformed_ranges, vec![
        Range {start: 50, end: 100},
        Range {start: 200, end: 225}
    ]);
}

#[test]
fn contained_to_after() {
    let input_range = Range {start: 125, end: 175};
    let map = DirtMap {
        name: String::from("one"),
        transforms: vec![
            DirtTransform {
                range: Range {start: 100, end: 150},
                transform: 100
            },
            DirtTransform {
                range: Range {start: 200, end: 250},
                transform: 1000
            }
        ]
    };
    let transformed_ranges = map.to_output_ranges(&input_range);

    assert_eq!(transformed_ranges, vec![
        Range {start: 225, end: 250},
        Range {start: 150, end: 175}
    ]);
}

#[test]
fn complete_overlap() {
    let input_range = Range {start: 50, end: 300};
    let map = DirtMap {
        name: String::from("one"),
        transforms: vec![
            DirtTransform {
                range: Range {start: 100, end: 150},
                transform: 100
            },
            DirtTransform {
                range: Range {start: 200, end: 250},
                transform: 1000
            }
        ]
    };
    let transformed_ranges = map.to_output_ranges(&input_range);

    assert_eq!(transformed_ranges, vec![
        Range {start: 50, end: 100},
        Range {start: 200, end: 250},
        Range {start: 150, end: 200},
        Range {start: 1200, end: 1250},
        Range {start: 250, end: 300},
    ]);
}
