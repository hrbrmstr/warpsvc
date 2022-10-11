use std::convert::Infallible;
use warp::Filter;
use cmpco::get_outages;

async fn outage_info() -> Result<impl warp::Reply, Infallible> {
	let outages = get_outages();
	let res: Vec<String> = outages.await.into_iter().map(|outage| serde_json::to_string(&outage).unwrap()).collect();
	Ok(warp::reply::with_header(res.join("\n"), "content-type", "application/json"))
}

#[tokio::main]
async fn main() {
	let cmp = warp::path("cmp").and_then(outage_info);

	let routes = warp::get().and(cmp);

	warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
