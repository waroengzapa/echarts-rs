use std::fmt::Display;

// #[derive(Clone)]
pub enum Theme {
    Light,
    Dark
}
// impl Theme {
//     pub fn to_string(&self) -> String {
//         match self {
//             Theme::Light => "light",
//             Theme::Dark => "dark",
//         }.to_string()
//     }
// }
impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}