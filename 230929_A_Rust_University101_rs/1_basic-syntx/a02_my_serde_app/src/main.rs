use serde::Deserialize;
use serde::Serialize;

fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1, 
                "title": "Hello, Rust"
            }
        "#,
    )
}

#[derive(Debug, Deserialize, Serialize)]
struct BlogPost {
    id: u32,
    title: String,
}

// impl fmt::Display for BlogPost {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.id, self.title)
//     }
// }

fn main() -> anyhow::Result<()> {
    let post: BlogPost = {
        let data = fetch_data();
        serde_json::from_str::<BlogPost>(&data)?
    };
    println!("deserialized = {post:?}");

    let post_json: String = serde_json::to_string(&post)?;
    println!("serialized = {post_json:?}");
    Ok(())
}
