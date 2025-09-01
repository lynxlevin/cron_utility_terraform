use chrono::{Datelike, NaiveDate, Weekday};
use crate::trash::Trash;

fn get_trash_schedule(date: NaiveDate) -> Trash {
    let weekday = date.weekday();
    let week_number = _nth_week(date);

    match weekday {
        Weekday::Sun => {
            match week_number {
                1 | 3 => Trash::PaperAndCloth,
                _ => Trash::None
            }
        },
        Weekday::Mon => Trash::Combustibles,
        Weekday::Tue => {
            match week_number {
                1 | 3 => Trash::PaperAndCloth,
                2 | 4 => Trash::PlasticBottles,
                _ => Trash::None
            }
        }
        Weekday::Wed => {
            match week_number {
                1 | 3 => Trash::CansAndBottles,
                2 | 4 => Trash::InCombustibles,
                _ => Trash::None
            }
        }
        Weekday::Thu => Trash::Combustibles,
        Weekday::Fri => Trash::Plastics,
        _ => Trash::None,
    }
}

fn _nth_week(date: NaiveDate) -> u32 {
    (date.day0() + 1).div_ceil(7)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[tokio::test]
    async fn test_get_trash_schedule() {
        let cases = vec![
            ("2025-03-01", Trash::None),
            ("2025-03-02", Trash::PaperAndCloth),
            ("2025-03-03", Trash::Combustibles),
            ("2025-03-04", Trash::PaperAndCloth),
            ("2025-03-05", Trash::CansAndBottles),
            ("2025-03-06", Trash::Combustibles),
            ("2025-03-07", Trash::Plastics),
            ("2025-03-08", Trash::None),
            ("2025-03-09", Trash::None),
            ("2025-03-10", Trash::Combustibles),
            ("2025-03-11", Trash::PlasticBottles),
            ("2025-03-12", Trash::InCombustibles),
            ("2025-03-13", Trash::Combustibles),
            ("2025-03-14", Trash::Plastics),
            ("2025-03-15", Trash::None),
            ("2025-03-16", Trash::PaperAndCloth),
            ("2025-03-17", Trash::Combustibles),
            ("2025-03-18", Trash::PaperAndCloth),
            ("2025-03-19", Trash::CansAndBottles),
            ("2025-03-20", Trash::Combustibles),
            ("2025-03-21", Trash::Plastics),
            ("2025-03-22", Trash::None),
            ("2025-03-23", Trash::None),
            ("2025-03-24", Trash::Combustibles),
            ("2025-03-25", Trash::PlasticBottles),
            ("2025-03-26", Trash::InCombustibles),
            ("2025-03-27", Trash::Combustibles),
            ("2025-03-28", Trash::Plastics),
            ("2025-03-29", Trash::None),
            ("2025-03-30", Trash::None),
            ("2025-03-31", Trash::Combustibles),
        ];

        for (date, expected) in cases {
            dbg!(&date);
            let result = get_trash_schedule(NaiveDate::from_str(date).unwrap());
            assert_eq!(expected, result);
        }
    }
}
