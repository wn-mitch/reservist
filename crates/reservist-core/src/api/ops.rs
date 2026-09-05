use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum View {
    Book,
    Fomc,
    Operations,
    Statement,
    Wire,
    Review,
    Routing,
    Request,
    Calendar,
    Folder,
    Scorecard,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RequestTiming {
    Normal,
    Accelerated,
    Declined,
    Missed,
}
impl RequestTiming {
    pub(crate) fn mode(self) -> crate::staff::RequestMode {
        match self {
            Self::Normal => crate::staff::RequestMode::Normal,
            Self::Accelerated => crate::staff::RequestMode::Accelerated,
            Self::Declined => crate::staff::RequestMode::Declined,
            Self::Missed => crate::staff::RequestMode::Missed,
        }
    }
}
