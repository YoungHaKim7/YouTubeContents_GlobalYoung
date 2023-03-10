// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: luhn
pub fn luhn(cc_number: &str) -> bool {
    // ANCHOR_END: luhn
    let mut digits_seen = 0;
    let mut sum = 0;
    for (i, ch) in cc_number.chars().rev().filter(|&ch| ch != ' ').enumerate() {
        match ch.to_digit(10) {
            Some(d) => {
                sum += if i % 2 == 1 {
                    let dd = d * 2;
                    dd / 10 + dd % 10
                } else {
                    d
                };
                digits_seen += 1;
            }
            None => return false,
        }
    }
    if digits_seen < 2 {
        return false;
    }

    sum % 10 == 0
}

fn main() {
    let cc_number = "1234 5678 1234 5670";
    println!(
        "Is {} a valid credit card number? {}",
        cc_number,
        if luhn(cc_number) { "yes" } else { "no" }
    );
}

// ANCHOR: unit-tests
#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(!luhn(" 0 0 "));
}

#[test]
fn test_valid_digit_cc_number() {
    assert!(!luhn("1234 5678 7890 0997"));
    assert!(!luhn("5678 8902 4561 9210"));
    assert!(!luhn("1212 5656 7890"));
}

#[test]
fn test_invalid_digit_cc_number() {
    assert!(!luhn("1234 5678 7890 0997"));
    assert!(!luhn("5678 8902 4561 9210"));
    assert!(!luhn("1212 5656 7890 0997"));
}

// ANCHOR_END: unit-tests
