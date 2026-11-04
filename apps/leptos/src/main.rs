#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
  use axum::Router;
  use axum::routing::get;
  use leptos::config::get_configuration;
  use leptos_axum::{LeptosRoutes, generate_route_list};
  use outfitters_leptos::app::{App, api_products, shell};

  let options = get_configuration(None).expect("leptos configuration").leptos_options;
  let addr = options.site_addr;
  let routes = generate_route_list(App);
  let app = Router::new()
    .route("/api/products", get(api_products))
    .leptos_routes(&options, routes, {
      let options = options.clone();
      move || shell(options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .with_state(options);
  let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
  axum::serve(listener, app.into_make_service()).await.expect("serve");
}

#[cfg(not(feature = "ssr"))]
fn main() {}
