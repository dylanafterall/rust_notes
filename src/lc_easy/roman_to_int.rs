use std::collections::HashMap;

// match expression implementation, iterating through string in reverse
// -----------------------------------------------------------------------------
pub fn roman_to_int_1(s: String) -> i32 {
    let mut result = 0;

    let mut prev = 0;
    for numeral in s.chars().rev() {
        let val = match numeral {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("Only valid roman numerals should be provided"),
        };
        if val < prev {
            result -= val;
        } else {
            result += val;
        }
        prev = val;
    }

    result
}

// match expression implementation using .as_bytes()
//  iterates forward through string bytes as opposed to first solution above
// -----------------------------------------------------------------------------
pub fn roman_to_int_2(s: String) -> i32 {
    let mut result = 0;

    let mut prev = 0;
    for &byte in s.as_bytes() {
        result += match byte {
            b'I' => 1,
            b'V' if prev == b'I' => 3,
            b'V' => 5,
            b'X' if prev == b'I' => 8,
            b'X' => 10,
            b'L' if prev == b'X' => 30,
            b'L' => 50,
            b'C' if prev == b'X' => 80,
            b'C' => 100,
            b'D' if prev == b'C' => 300,
            b'D' => 500,
            b'M' if prev == b'C' => 800,
            b'M' => 1000,
            _ => panic!("Only valid roman numerals should be provided"),
        };
        prev = byte;
    }

    result
}

// HashMap implementation
// -----------------------------------------------------------------------------
pub fn roman_to_int_3(s: String) -> i32 {
    let roman_to_int_map: HashMap<char, i32> = vec![
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .into_iter()
    .collect();

    let mut result = 0;

    let mut prev = 0;
    for numeral in s.chars() {
        if let Some(&val) = roman_to_int_map.get(&numeral) {
            if val > prev {
                result += val - prev - prev;
            } else {
                result += val
            }
            prev = val;
        } else {
            panic!("Only valid roman numerals should be provided")
        }
    }

    result
}
