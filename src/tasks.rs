use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    pub name: String,
    pub command: String
}

#[derive(Debug, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>
}

pub fn get_tasks(raw_toml: String) {
    let tasks = toml::from_str::<Tasks>(&raw_toml);

    match tasks {
        Ok(tasks) => println!("{:?}", tasks),
        Err(e) => eprintln!("{}", e)
    }
}
