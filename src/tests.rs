#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use anyhow::Context;
    use std::fs::read_to_string;

    macro_rules! day_test {
        ($day:literal, $part1:literal) => {
            paste::item! {
                #[test]
                fn [<day_$day>] () {
                    let input = read_input($day);
                    let day = crate::[<day_$day>]::Day {};

                    let answer = $part1;
                    let result = day.compute_1(&input).unwrap();
                    assert_eq!(result, answer);
                }
            }
        };
        ($day:literal, $part1:literal, $part2:literal) => {
            paste::item! {
                #[test]
                fn [<day_$day>] () {
                    let input = read_input($day);
                    let day = crate::[<day_$day>]::Day {};

                    let answer = $part1;
                    let result = day.compute_1(&input).unwrap();
                    assert_eq!(result, answer);

                    let answer = $part2;
                    let result = day.compute_2(&input).unwrap();
                    assert_eq!(result, answer);
                }
            }
        };
    }

    fn read_input(day: &str) -> String {
        let file = format!("./input/{day}");

        read_to_string(&file)
            .context(format!("Failed to read {file}"))
            .unwrap()
    }

    day_test!("01", "55208", "54578");
    day_test!("02", "2683", "49710");
    day_test!("03", "531561", "83279367");
    day_test!("04", "22193", "5625994");
    day_test!("05", "825516882" /*, "136096660" */);
    day_test!("06", "1108800", "36919753");
    day_test!("07", "250058342", "250506580");
    day_test!("08", "13207");
}
