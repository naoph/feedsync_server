use crate::PgPool;

#[derive(Clone)]
pub struct State {
    pool: PgPool,
}

impl State {
    pub fn new(pool: PgPool) -> State {
        State { pool }
    }
}
