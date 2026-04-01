# Backend API Documentation

> REST API for Recipe Management built with Axum and Sled

## Base URL

```
Development: http://127.0.0.1:3001
Production: TBD
```

## Authentication

⚠️ Currently, the API has **no authentication**. All endpoints are publicly accessible.

## CORS Policy

The API allows **all origins** in development mode (`allow_origin(Any)`). This must be restricted before production deployment.

## Data Models

### Recipe

```typescript
interface Recipe {
  id: number;           // Auto-generated, sequential
  title: string;        // Recipe name
  description: string;  // Short description
  image: string;        // Image URL
  isFavorite: boolean;  // Favorite status
}
```

**Rust Struct** (with serde serialization):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Recipe {
    id: u32,
    title: String,
    description: String,
    image: String,
    #[serde(rename = "isFavorite")]  // Converts to camelCase for JSON
    is_favorite: bool,
}
```

## Endpoints

### GET /recipes

Retrieves all recipes from the database, sorted by ID.

**Request**
```http
GET /recipes HTTP/1.1
Host: 127.0.0.1:3001
```

**Response**

Status: `200 OK`

```json
[
  {
    "id": 1,
    "title": "Spaghetti Bolognese",
    "description": "A classic Italian meat sauce pasta.",
    "image": "https://loremflickr.com/600/400/spaghetti,bolognese",
    "isFavorite": false
  },
  {
    "id": 2,
    "title": "Chicken Tikka Masala",
    "description": "Creamy tomato-based Indian curry.",
    "image": "https://loremflickr.com/600/400/chicken,curry",
    "isFavorite": true
  }
]
```

**cURL Example**
```bash
curl http://127.0.0.1:3001/recipes
```

**JavaScript Example**
```typescript
const response = await fetch('http://127.0.0.1:3001/recipes');
const recipes = await response.json();
```

**Errors**

| Status Code | Description |
|-------------|-------------|
| 200 | Success |
| 500 | Database error (server panics) |

**Implementation Details**:
- Iterates through all keys in Sled database with prefix `recipe:`
- Deserializes each value from JSON bytes to Recipe struct
- Sorts by ID for consistent ordering
- Returns empty array `[]` if no recipes exist

---

### POST /recipes

Creates a new recipe with an auto-generated ID.

**Request**

```http
POST /recipes HTTP/1.1
Host: 127.0.0.1:3001
Content-Type: application/json

{
  "title": "Margherita Pizza",
  "description": "Classic Neapolitan pizza with fresh basil.",
  "image": "https://loremflickr.com/600/400/pizza,margherita",
  "isFavorite": false
}
```

**Notes**:
- `id` field is **ignored** if provided - the server generates IDs
- All other fields are **required**

**Response**

Status: `200 OK`

```json
{
  "id": 10,
  "title": "Margherita Pizza",
  "description": "Classic Neapolitan pizza with fresh basil.",
  "image": "https://loremflickr.com/600/400/pizza,margherita",
  "isFavorite": false
}
```

**cURL Example**
```bash
curl -X POST http://127.0.0.1:3001/recipes \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Margherita Pizza",
    "description": "Classic Neapolitan pizza",
    "image": "https://example.com/pizza.jpg",
    "isFavorite": false
  }'
```

**JavaScript Example**
```typescript
const newRecipe = await fetch('http://127.0.0.1:3001/recipes', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
  },
  body: JSON.stringify({
    title: 'Margherita Pizza',
    description: 'Classic Neapolitan pizza',
    image: 'https://example.com/pizza.jpg',
    isFavorite: false
  })
});

const recipe = await newRecipe.json();
console.log('Created recipe with ID:', recipe.id);
```

**Errors**

| Status Code | Description |
|-------------|-------------|
| 200 | Success |
| 400 | Invalid JSON or missing required fields |
| 500 | Database error (server panics) |

**ID Generation Algorithm**:
1. Query all existing recipe keys from database
2. Extract ID from each key (format: `recipe:{id}`)
3. Find maximum ID
4. New ID = max ID + 1 (or 1 if database empty)

⚠️ **Known Issue**: Race condition if two clients POST simultaneously. Both may calculate the same ID, resulting in the last write winning and one recipe being lost. Not safe for production without proper locking or atomic ID generation.

---

## Database Schema

### Storage Format

**Key**: `recipe:{id}` (bytes)
```
Example: b"recipe:1", b"recipe:2", etc.
```

**Value**: JSON-serialized Recipe (bytes)
```json
{"id":1,"title":"...","description":"...","image":"...","isFavorite":false}
```

### Database Operations

**Read All Recipes**:
```rust
for item in db.iter() {
    if let Ok((key, value)) = item {
        if key.starts_with(b"recipe:") {
            let recipe = serde_json::from_slice::<Recipe>(&value)?;
            // Process recipe
        }
    }
}
```

**Insert Recipe**:
```rust
let key = format!("recipe:{}", recipe.id);
let value = serde_json::to_vec(&recipe)?;
db.insert(key.as_bytes(), value)?;
```

---

## Error Handling

⚠️ **Current Implementation**: Most errors cause server panics (`.expect()` calls)

**What happens on error**:
- Database corrupted → Server crashes on startup
- Serialization fails → Server panics and restarts
- Invalid JSON in POST → Axum returns 400 automatically

**Better Approach** (not yet implemented):
```rust
async fn get_recipes(State(state): State<AppState>) 
    -> Result<Json<Vec<Recipe>>, StatusCode> 
{
    // Return proper HTTP error codes instead of panicking
    db.get(key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
}
```

---

## Initialization & Seeding

### Automatic Seeding

On first run, the database is seeded from `recipes.json` (embedded at compile time):

**Seed Logic**:
```rust
async fn seed_database(db: &sled::Db) {
    if db.len() > 0 {
        println!("✅ Database already seeded");
        return;  // Skip if data exists
    }
    
    // Load JSON from compile-time embedded file
    let recipes_json = include_str!("../../frontend/src/resources/recipes.json");
    let recipes: Vec<Recipe> = serde_json::from_str(recipes_json)?;
    
    // Insert each recipe
    for recipe in recipes {
        let key = format!("recipe:{}", recipe.id);
        db.insert(key.as_bytes(), serde_json::to_vec(&recipe)?)?;
    }
    
    println!("✅ Seeded {} recipes", db.len());
}
```

**Important Notes**:
- Seeding only happens if database is **completely empty**
- Changing `recipes.json` after first run **won't update** the database
- To re-seed: delete the `recipes.db/` folder and restart

### Manual Database Reset

```bash
# Stop the server
# Delete the database
rm -rf packages/backend/recipes.db

# Restart the server
cd packages/backend
cargo run

# Output: ✅ Seeded database with 9 recipes
```

---

## Performance Characteristics

### Database Performance

- **Read**: O(log n) due to B-tree structure in Sled
- **Write**: O(log n) plus disk I/O
- **Memory**: Database loads keys into memory for fast iteration
- **Concurrency**: Sled handles concurrent reads safely, but writes may conflict

### API Performance

- **GET /recipes**: ~1-5ms for <1000 recipes (mostly serialization)
- **POST /recipes**: ~2-10ms (ID generation + write + fsync)
- **Concurrency**: Tokio handles thousands of concurrent connections

**Bottlenecks**:
1. ID generation iterates all keys (O(n)) - could be optimized with atomic counter
2. GET /recipes returns all recipes at once - no pagination

---

## Testing the API

### Using cURL

**Get all recipes**:
```bash
curl http://127.0.0.1:3001/recipes | jq '.'
```

**Get first 2 recipes**:
```bash
curl http://127.0.0.1:3001/recipes | jq '.[0:2]'
```

**Create a recipe**:
```bash
curl -X POST http://127.0.0.1:3001/recipes \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Test Recipe",
    "description": "Testing the API",
    "image": "https://example.com/test.jpg",
    "isFavorite": true
  }' | jq '.'
```

### Using Postman

1. **Import Collection** (optional - create manually):
   - Method: GET
   - URL: `http://127.0.0.1:3001/recipes`
   - Save as "Get All Recipes"

2. **Create Recipe Request**:
   - Method: POST
   - URL: `http://127.0.0.1:3001/recipes`
   - Headers: `Content-Type: application/json`
   - Body (raw JSON):
     ```json
     {
       "title": "{{$randomWord}} {{$randomWord}}",
       "description": "{{$randomLoremSentence}}",
       "image": "https://loremflickr.com/600/400/food",
       "isFavorite": false
     }
     ```

### Using JavaScript (Browser Console)

```javascript
// Fetch all recipes
const recipes = await fetch('http://127.0.0.1:3001/recipes')
  .then(r => r.json());
console.table(recipes);

// Create a recipe
const newRecipe = await fetch('http://127.0.0.1:3001/recipes', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    title: 'Browser Test',
    description: 'Created from browser',
    image: 'https://example.com/test.jpg',
    isFavorite: false
  })
}).then(r => r.json());
console.log('Created:', newRecipe);
```

---

## Future Endpoints (Not Yet Implemented)

### PATCH /recipes/:id

Update an existing recipe.

**Proposed Request**:
```http
PATCH /recipes/5 HTTP/1.1
Content-Type: application/json

{
  "isFavorite": true
}
```

### DELETE /recipes/:id

Delete a recipe by ID.

**Proposed Request**:
```http
DELETE /recipes/5 HTTP/1.1
```

### GET /recipes/:id

Get a single recipe by ID.

**Proposed Request**:
```http
GET /recipes/5 HTTP/1.1
```

---

## Deployment Considerations

### Environment Configuration

⚠️ **TODO**: Make these configurable via environment variables

```rust
// Hardcoded (current)
let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;

// Configurable (recommended)
let port = env::var("PORT").unwrap_or("3001".to_string());
let addr = format!("0.0.0.0:{}", port);
let listener = tokio::net::TcpListener::bind(&addr).await?;
```

### CORS Configuration

```rust
// Development (current)
.allow_origin(Any)

// Production (required)
.allow_origin("https://your-frontend-domain.com".parse::<HeaderValue>()?)
```

### Database Path

```rust
// Development
sled::open("recipes.db")?

// Production (use environment variable)
let db_path = env::var("DATABASE_PATH").unwrap_or("recipes.db".to_string());
sled::open(db_path)?
```

---

*Last updated: 2026-03-31 | Source: Backend implementation in main.rs and teacher session logs*
