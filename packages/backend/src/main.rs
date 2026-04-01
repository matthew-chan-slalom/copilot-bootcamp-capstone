use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Recipe {
    id: u32,
    title: String,
    description: String,
    image: String,
    #[serde(rename = "isFavorite")]
    is_favorite: bool,
}

#[derive(Clone)]
struct AppState {
    db: sled::Db,
}

#[tokio::main]
async fn main() {
    // Open the database
    let db = sled::open("recipes.db").expect("Failed to open database");
    
    // Seed initial data from recipes.json
    seed_database(&db).await;
    
    let state = AppState { db };
    
    // Configure CORS for frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build the router
    let app = Router::new()
        .route("/recipes", get(get_recipes))
        .route("/recipes", post(create_recipe))
        .layer(cors)
        .with_state(state);
    
    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3001");
    
    axum::serve(listener, app).await.unwrap();
}

async fn seed_database(db: &sled::Db) {
    // Check if database is already seeded
    if db.len() > 0 {
        println!("✅ Database already seeded with {} recipes", db.len());
        return;
    }
    
    // Load recipes from JSON file
    let recipes_json = include_str!("../../frontend/src/resources/recipes.json");
    let recipes: Vec<Recipe> = serde_json::from_str(recipes_json)
        .expect("Failed to parse recipes.json");
    
    // Insert each recipe into the database
    for recipe in recipes {
        let key = format!("recipe:{}", recipe.id);
        let value = serde_json::to_vec(&recipe).expect("Failed to serialize recipe");
        db.insert(key.as_bytes(), value)
            .expect("Failed to insert recipe");
    }
    
    println!("✅ Seeded database with {} recipes", db.len());
}

async fn get_recipes(State(state): State<AppState>) -> Json<Vec<Recipe>> {
    let mut recipes = Vec::new();
    
    for item in state.db.iter() {
        if let Ok((key, value)) = item {
            if key.starts_with(b"recipe:") {
                if let Ok(recipe) = serde_json::from_slice::<Recipe>(&value) {
                    recipes.push(recipe);
                }
            }
        }
    }
    
    // Sort by id for consistent ordering
    recipes.sort_by_key(|r| r.id);
    
    Json(recipes)
}

async fn create_recipe(
    State(state): State<AppState>,
    Json(mut recipe): Json<Recipe>,
) -> Json<Recipe> {
    // Generate new ID (simple auto-increment based on max existing ID)
    let max_id = state
        .db
        .iter()
        .filter_map(|item| {
            if let Ok((key, _)) = item {
                if key.starts_with(b"recipe:") {
                    let id_str = String::from_utf8_lossy(&key[7..]);
                    id_str.parse::<u32>().ok()
                } else {
                    None
                }
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);
    
    recipe.id = max_id + 1;
    
    let key = format!("recipe:{}", recipe.id);
    let value = serde_json::to_vec(&recipe).expect("Failed to serialize recipe");
    
    state
        .db
        .insert(key.as_bytes(), value)
        .expect("Failed to insert recipe");
    
    Json(recipe)
}
