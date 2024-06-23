mod project_data {
    use serde::{Deserialize, Serialize};

    struct Project {
        id: String,
        name: String,
        description: String,
    }
}

fn main() {
    println!("Hello, world!");
}
