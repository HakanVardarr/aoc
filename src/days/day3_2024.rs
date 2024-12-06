use crate::{day, Error, InputReciever, Solver};

day!(Day3_2024, 3, 2024);
impl Solver for Day3_2024 {
    fn first_part(&self) -> String {
        let mut result: u32 = 0;

        let chars = self.input.chars().map(|c| c as u8).collect::<Vec<u8>>();
        for i in 0..chars.len() {
            let mut end = i + 4;
            if end < chars.len() {
                let token = String::from_utf8(chars[i..end].to_vec()).unwrap();

                if token == String::from("mul(") {
                    let start = end;
                    while end < chars.len() && chars[end].is_ascii_digit() {
                        end += 1;
                    }

                    if chars[end] == ',' as u8 {
                        let first_number = String::from_utf8(chars[start..end].to_vec())
                            .unwrap()
                            .parse::<u32>()
                            .unwrap();

                        end += 1;
                        let start = end;
                        while end < chars.len() && chars[end].is_ascii_digit() {
                            end += 1;
                        }

                        if chars[end] == ')' as u8 {
                            let second_number = String::from_utf8(chars[start..end].to_vec())
                                .unwrap()
                                .parse::<u32>()
                                .unwrap();

                            result += first_number * second_number;
                        }
                    }
                }
            }
        }

        result.to_string()
    }

    fn second_part(&self) -> String {
        let mut result: u32 = 0;

        let chars = self.input.chars().map(|c| c as u8).collect::<Vec<u8>>();
        let mut multiply = true;
        for i in 0..chars.len() {
            let mut end = i + 4;
            if end < chars.len() {
                let token = String::from_utf8(chars[i..end].to_vec()).unwrap();

                if token == String::from("mul(") {
                    let start = end;
                    while end < chars.len() && chars[end].is_ascii_digit() {
                        end += 1;
                    }

                    if chars[end] == ',' as u8 {
                        let first_number = String::from_utf8(chars[start..end].to_vec())
                            .unwrap()
                            .parse::<u32>()
                            .unwrap();

                        end += 1;
                        let start = end;
                        while end < chars.len() && chars[end].is_ascii_digit() {
                            end += 1;
                        }

                        if chars[end] == ')' as u8 && multiply {
                            let second_number = String::from_utf8(chars[start..end].to_vec())
                                .unwrap()
                                .parse::<u32>()
                                .unwrap();

                            result += first_number * second_number;
                        }
                    }
                } else if token == String::from("do()") {
                    multiply = true;
                } else if token == String::from("don'") {
                    let token = String::from_utf8(chars[i..end + 3].to_vec()).unwrap();
                    if token == String::from("don't()") {
                        multiply = false;
                    }
                }
            }
        }

        result.to_string()
    }
}
