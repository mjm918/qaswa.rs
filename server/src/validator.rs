//! HTTP request validation module

use serde_json::json;
use validator::Validate;
use utility::app_error;
use utility::errors::AppResult;

/// Validate the HTTP request parameters
pub fn validate_request_data<T: Validate>(data: &T) -> AppResult<()> {
	match data.validate() {
		Ok(_) => Ok(()),
		Err(errors) => Err(app_error!(AppErrorCode::BadRequest, json!(errors).to_string())),
	}
}