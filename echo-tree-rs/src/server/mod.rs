use warp::Filter;

use self::security::check_auth;

mod security;

async fn test_echo() -> Result<String, warp::Rejection> {
  Ok(format!("echo: hello"))
}

pub fn test_filter() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
  warp::path("test")
    .and(warp::get())
    .and(warp::path::end())
    .and(check_auth().untuple_one())
    .and_then(test_echo)
}

pub async fn server() {
  let register = warp::path("register")
    .and(warp::path::param())
    .and(warp::header("user-agent"))
    .and(check_auth().untuple_one())
    .map(|param: String, agent: String| {
      format!("register: {} with user-agent: {}", param, agent)
    });

  // warp server
  let routes = test_filter().or(register);
  warp::serve(routes).run(([127,0,0,1], 2121)).await;
}