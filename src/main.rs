use std::env;

use actix_web::{cookie::time::Duration, get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2, PasswordHash, PasswordVerifier};
use dotenv::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{postgres::PgPoolOptions, types::chrono::Utc, Pool, Postgres};
use serde::{Deserialize,Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
struct UserRegister{
    name:String,
    email:String,
    password:String,
}

#[derive(Debug,Deserialize,Serialize)]
struct Claims{
    sub:String,
    exp:usize,
}

fn create_jwt(user_id:&str,secret:&str)->Result<String,jsonwebtoken::errors::Error>{
    let expiration = Utc::now()
    .checked_add_signed(Duration::hours(12))
    .expect("valid timestamp")
    .timestamp();

    let claims = Claims{
        sub : user_id.to_string(),
        exp : expiration as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()),
    )
}
#[derive(Deserialize,Serialize)]
struct Login{
    email:String,
    password:String,
}
#[post("/register")]
async fn register(pool:web::Data<Pool<Postgres>>,info:web::Data<UserRegister>)-> impl Responder{
    let salt = SaltString::generate(&mut OsRng);
    let argon2=Argon2::default();
    let hashpassword=match  argon2.hash_password(info.password.as_bytes(), &salt){
        Ok(hash)=>hash.to_string(),
        Err(_)=>return HttpResponse::InternalServerError().body("Password could not be hasshed")
    };
    let user_id = Uuid::new_v4();
    let now = Utc::now();
    let sql = include_str!("queries/register.sql");
    let result = sqlx::query(sql)
    .bind(user_id)
    .bind(&info.name)
    .bind(&info.email)
    .bind(&hashpassword)
    .bind(now)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_)=>HttpResponse::Ok().body("User Register Successfully"),
        Err(err)=>HttpResponse::InternalServerError().body(format!("There is some error: {}",err.to_string())),
    }
}

#[post("/login")]
async fn login(
    pool: web::Data<Pool<Postgres>>,
    info: web::Json<Login>,
) -> impl Responder {
    
    let sql = include_str!("queries/login.sql");

    
    let result = sqlx::query_as::<_, (String,)>(sql)
        .bind(&info.email)
        .fetch_one(pool.get_ref())
        .await;

    let (stored_hash,) = match result {
        Ok(row) => row,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid email or password"),
    };

    
    let parsed_hash = match PasswordHash::new(&stored_hash) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().body("Invalid password hash format"),
    };

    let argon2 = Argon2::default();

    match argon2.verify_password(info.password.as_bytes(), &parsed_hash) {
        Ok(_) => HttpResponse::Ok().body("Login successful "),
        Err(_) => HttpResponse::Unauthorized().body("Invalid email or password "),
    }
}


#[get("/index")]
async fn index()-> impl Responder{
    HttpResponse::Ok().body("This is the main page")
}
async fn get_db()-> Pool<Postgres>{
    let database_url = env::var("DATABASE_URL").expect("Cant find the database url in teh env file");
    let pool : Pool<Postgres>= PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await.expect("Cant connect to the database");

    return pool;
}
#[actix_web::main]
async fn main()-> std::io::Result<()>{
    dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none(){
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    let pool = get_db().await;
    env_logger::init();
    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(index)
        .wrap(Logger::default())
        .service(register)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}