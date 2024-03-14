use warp::Filter;

pub fn check_auth() -> impl Filter<Extract = ((),), Error = warp::Rejection> + Clone {
  warp::any()
    .and(warp::header::<String>("X-Auth-Token"))
    .and_then(
      |xauth: String| async move {
        // check auth
        if xauth != "123456" {
          return Err(warp::reject::reject());
        }

        return Ok::<_, warp::Rejection>(());
      })
}