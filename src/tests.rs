#[cfg(test)]
mod tests {
    use crate::solution::Solution;
    use anyhow::Context;
    use std::fs::read_to_string;

    fn read_input(day: &str) -> String {
        let file = format!("./input/{day}");

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
        let input = read_input("02");
        let day = crate::day_02::Day {};

        let answer = "2683";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "49710";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_04() {
        let input = read_input("04");
        let day = crate::day_04::Day {};

        let answer = "22193";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "5625994";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }

    #[test]
    fn day_05() {
        let input = read_input("05");
        let day = crate::day_05::Day {};

        let answer = "825516882";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        // NOTE: Disabled because this test is heavy to run
        // let answer = "136096660";
        // let result = day.compute_2(&input).unwrap();
        // assert_eq!(result, answer);
    }

    #[test]
    fn day_06() {
        let input = read_input("06");
        let day = crate::day_06::Day {};

        let answer = "1108800";
        let result = day.compute_1(&input).unwrap();
        assert_eq!(result, answer);

        let answer = "36919753";
        let result = day.compute_2(&input).unwrap();
        assert_eq!(result, answer);
    }
}
