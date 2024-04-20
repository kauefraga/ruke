use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub command: String,
    pub arguments: Option<Vec<String>>
}

#[derive(Deserialize)]
pub struct Rukefile {
    pub tasks: Vec<Recipe>
}

impl Rukefile {
    pub fn from_str(raw: &str) -> Option<Rukefile> {
        let rukefile = toml::from_str::<Rukefile>(raw);

        match rukefile {
            Ok(rukefile) => Some(rukefile),
            Err(_) => None
        }
    }
}
