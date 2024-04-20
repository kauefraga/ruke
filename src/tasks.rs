use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub command: String,
    pub arguments: Option<Vec<String>>
}

#[derive(Clone, Deserialize)]
pub struct Rukefile {
    pub tasks: Vec<Recipe>
}

impl Rukefile {
    pub fn from_str(raw: &str) -> Result<Self, toml::de::Error> {
        toml::from_str::<Rukefile>(raw)
    }

    pub fn find_recipe(&self, name: String) -> Option<Recipe> {
        let recipe = self.tasks
            .iter()
            .find(|recipe| recipe.name.eq(&name));

        return recipe.cloned();
    }
}
