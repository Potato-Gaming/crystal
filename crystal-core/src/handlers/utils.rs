use gotham::helpers::http::response::create_empty_response;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};

pub fn empty_handler(state: State) -> (State, Response<Body>) {
    let res = create_empty_response(&state, StatusCode::NO_CONTENT);

    (state, res)
}
