use std::time::Instant;

use axum::{extract::MatchedPath, middleware::Next, response::IntoResponse};
use hyper::Request;
use metrics::{histogram, increment_counter};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use tracing::error;
use utility::app_error;
use utility::errors::{AppResult,AppErrorCode,AppError};
use crate::{APP_NAME, SECONDS_DURATION_BUCKETS};

pub struct PrometheusMetric {}

impl PrometheusMetric {
	/// Return a new `PrometheusHandle`
	pub fn get_handle() -> AppResult<PrometheusHandle> {
		PrometheusBuilder::new()
			.set_buckets_for_metric(
				Matcher::Full("http_requests_duration_seconds".to_string()),
				SECONDS_DURATION_BUCKETS,
			)
			.map_err(|err| app_error!(AppErrorCode::InternalError, err.to_string()))?
			.install_recorder()
			.map_err(|err| app_error!(AppErrorCode::InternalError, err.to_string()))
	}

	/// Layer tracking requests
	pub async fn get_layer<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
		let start = Instant::now();
		let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
			matched_path.as_str().to_owned()
		} else {
			req.uri().path().to_owned()
		};
		let method = req.method().clone();

		let response = next.run(req).await;

		let latency = start.elapsed().as_secs_f64();
		let status = response.status().as_u16().to_string();
		let labels = [
			("method", method.to_string()),
			("path", path.to_owned()),
			("service", APP_NAME.to_owned()),
			("status", status),
		];
		increment_counter!(format!("http_request{}",method.to_string()), &labels);
		increment_counter!(format!("http_path{}",path.to_owned()), &labels);
		increment_counter!("http_requests_total", &labels);

		histogram!(format!("http{}_requests_duration_seconds",path.to_owned()), latency.clone(), &labels);
		histogram!("http_requests_duration_seconds", latency.to_owned(), &labels);

		response
	}
}