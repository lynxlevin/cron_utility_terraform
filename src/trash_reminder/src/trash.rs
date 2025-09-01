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