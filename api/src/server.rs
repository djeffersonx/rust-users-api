use models::repository::UserRepository;
use tide::{Response, Server, StatusCode};

use crate::handlers;
use crate::State;

pub(crate) async fn get_app(
    repository: Box<dyn UserRepository + Send + Sync + 'static>,
) -> tide::Result<Server<State>> {
    let state = State::new(repository).await?;
    let mut app = tide::with_state(state);

    app.at("/users").get(handlers::get_all);
    app.at("/users").post(handlers::save);
    app.at("/users/:id").get(handlers::get);

    app.at("/health")
        .get(|_| async { Ok(Response::new(StatusCode::Ok)) });

    Ok(app)
}
