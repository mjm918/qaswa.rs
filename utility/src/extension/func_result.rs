use flinch::doc::QueryBased;
use flinch::doc_trait::Document;
use flinch::headers::FuncResult;
use serde_json::{Map, Value};
use crate::extension::FuncResultExtractor;

impl FuncResultExtractor for FuncResult<Option<(String, QueryBased)>> {
	fn get_object(&self) -> Map<String, Value> {
		if let Some((_key, qb)) = &self.data {
			return qb.object().clone();
		}
		Map::new()
	}
}