use axum::routing::{get, post, Router};
mod recipe_handler;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| ("8080".to_string()));
    let addr = format!("0.0.0.0:{}", port);
    let mongodb_url = std::env::var("MONGODB_URI").expect("MONGODB_URL must be set");
    // println!("Connecting to {}", mongodb_url);
    let client_options =
        ClientOptions::parse_with_resolver_config(&mongodb_url, ResolverConfig::cloudflare())
            .await
            .unwrap();
    let client = Client::with_options(client_options).unwrap();
    // if let Err(e) = client.list_database_names(None, None).await {
    //     println!("Failed to list databases: {:?}", e);
    // }
    // for name in client.list_database_names(None, None).await? {
    //     println!("- {}", name);
    // }
    let db = client.database("Recipe");
    // for collection_name in db.list_collection_names(None).await? {
    //     println!("{}", collection_name);
    // }
    // let db = db.clone();
    // println!("Connected to database, {:?}", collections);
    println!("Listening on http:// {} ", addr);
    println!("Connected to database, {:?}", db.name());
    let app = Router::new()
        .route("/", get(recipe_handler::health_check))
        .route("/:id", get(recipe_handler::get_categories))
        .route("/", post(recipe_handler::create_category))
        .with_state(db);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
