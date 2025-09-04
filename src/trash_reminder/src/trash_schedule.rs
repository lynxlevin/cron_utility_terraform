use chrono::{Datelike, NaiveDate, Weekday};
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Trash {
    Combustibles,
    Plastics,
    PaperAndCloth,
    CansAndBottles,
    PlasticBottles,
    InCombustibles,
    None,
}

impl Display for Trash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Trash::Combustibles => write!(f, "燃やすゴミ"),
            Trash::Plastics => write!(f, "プラスチック"),
            Trash::PaperAndCloth => write!(f, "紙布"),
            Trash::CansAndBottles => write!(f, "缶瓶"),
            Trash::PlasticBottles => write!(f, "ペットボトル"),
            Trash::InCombustibles => write!(f, "小型不燃"),
            _ => write!(f, "無"),
        }
    }
}

pub fn get_trash_schedule(date: NaiveDate) -> Trash {
    let weekday = date.weekday();
    let week_number = date.day().div_ceil(7);

    match weekday {
        Weekday::Sun => match week_number {
            1 | 3 => Trash::PaperAndCloth,
            _ => Trash::None,
        },
        Weekday::Mon => Trash::Combustibles,
        Weekday::Tue => match week_number {
            1 | 3 => Trash::PaperAndCloth,
            2 | 4 => Trash::PlasticBottles,
            _ => Trash::None,
        },
        Weekday::Wed => match week_number {
            1 | 3 => Trash::CansAndBottles,
            2 | 4 => Trash::InCombustibles,
            _ => Trash::None,
        },
        Weekday::Thu => Trash::Combustibles,
        Weekday::Fri => Trash::Plastics,
        Weekday::Sat => Trash::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_trash_schedule() {
        for day in 1..32 {
            dbg!(&day);
            let date = NaiveDate::from_ymd_opt(2025, 3, day).unwrap();
            let result = get_trash_schedule(date);
            let expected = match day {
                3 | 6 | 10 | 13 | 17 | 20 | 24 | 27 | 31 => Trash::Combustibles,
                7 | 14 | 21 | 28 => Trash::Plastics,
                2 | 4 | 16 | 18 => Trash::PaperAndCloth,
                5 | 19 => Trash::CansAndBottles,
                11 | 25 => Trash::PlasticBottles,
                12 | 26 => Trash::InCombustibles,
                _ => Trash::None,
            };
            assert_eq!(expected, result);
        }
    }
}
