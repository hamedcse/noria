use serde_json;
use serde_json::{Value};
use nom_sql::parser as sql_parser;
use nom_sql::SqlQuery;

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct Policy {
	pub table: String,
    pub predicate: SqlQuery,
}

impl Policy {
	pub fn parse(policy_text: &str) -> Vec<Policy> {
		let policies: Vec<Value> = match serde_json::from_str(policy_text) {
			Ok(v) => v,
			Err(e) => panic!(e.to_string()),
		};

		policies.iter()
				.map(|p| {
					let pred = sql_parser::parse_query(
										format!("select * from {} {};", 
											p["table"].as_str().unwrap(), 
											p["predicate"].as_str().unwrap()
										).as_str()
								).unwrap();

					Policy {
						table: p["table"].to_string(),
						predicate: pred,
					}
				})
				.collect()
	}
}

mod tests {
	use super::*;

	#[test]
	fn it_parses() {
		let policy_text = r#"[{ "table": "posts", "predicate": "WHERE posts.type = ?" },
							  { "table": "posts", "predicate": "WHERE posts.author = ?" }]"#;

		let policies = Policy::parse(policy_text);

		assert_eq!(policies.len(), 2);
	}
}


