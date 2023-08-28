use std::sync::Arc;
use flinch::database::Database;
use flinch::doc::QueryBased;
use utility::env::Variables;
use crate::util::ConfigState;

pub type SharedState = Arc<State>;

pub struct State {
	pub env: Variables,
	pub flinch: Arc<Database<QueryBased>>,
	pub config: ConfigState,
}

impl State {
	pub fn init(env: Variables, flinch: Arc<Database<QueryBased>>) -> Self {
		Self { env: env.clone(), config: ConfigState::from(env), flinch }
	}
}