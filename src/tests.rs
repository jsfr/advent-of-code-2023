#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use anyhow::Context;
    use std::fs::read_to_string;

    fn read_input(day: &str) -> String {
        let file = format!("./input/{}", day);

        read_to_string(&file)
            .context(format!("Failed to read {file}"))
            .unwrap()
    }

    #[test]
    fn day_01() {
        let input = read_input("01");
        let day = crate::day_01::Day {};

        let answer = "55208";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "54578";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_02() {
        let input = read_input("01");
        let day = crate::day_02::Day {};

        let answer = "2683";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "49710";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }
}
