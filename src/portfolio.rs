use std::fmt::Display;

use serde::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Portfolio {
    pub projects: Vec<Project>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub title: String,
    pub description: String,

    pub images: Vec<Image>,

    pub status: Status,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Image {
    pub src: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Wip,
    Working,
    Stable,
}

impl Display for Status {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Wip => f.write_str("WIP"),
            Status::Working => f.write_str("Working"),
            Status::Stable => f.write_str("Stable"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Link {
    pub href: String,
    pub display: String,
    pub icon: LinkIcon,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum LinkIcon {
    #[default]
    Chain,
    GitHub,
}

impl LinkIcon {
    pub fn icon_src(&self) -> &'static str {
        match self {
            LinkIcon::Chain => "img/chain.svg",
            LinkIcon::GitHub => "img/github.svg",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portfolio_deserializes() {
        serde_json::from_str::<Portfolio>(include_str!("../portfolio.json")).unwrap();
    }
}
