use std:: env;
use chrono::{Duration as ChronoDuration};
use actix_web::{  get, post, web, App, HttpResponse, HttpServer, Responder};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{decode, encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{ postgres::PgPoolOptions, types::chrono::Utc, Pool, Postgres};
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct UserRegister{
    name:String,
    email:String,
    password:String,
}
#[derive(Deserialize,Debug)]
struct UserLogin{
    email:String,
    password:String,
}
#[derive(Serialize)]
struct Claims{
    sub:String,
    exp:usize,
}
async fn get_db()-> Result<Pool<Postgres>,sqlx::Error>{
    let database_url = env::var("DATABASE_URL").map_err(|_|sqlx::Error::Configuration(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Database url is not found"))))?;
    let pool = PgPoolOptions::new()
    .max_connections(10)
    .acquire_timeout(std::time::Duration::from_secs(5))
    .connect(&database_url)
    .await?;

    Ok(pool)
}
fn init_tracing(){
    fmt()
    
    .with_target(false)
    .compact()
    .init();
}
#[get("/index")]
async fn index()-> impl Responder{
    HttpResponse::Ok().body("This is the main page")
}
fn create_jwt(user_email:&str)->Result<String,JwtError>{
    let expiration = Utc::now()
    .checked_add_signed(ChronoDuration::hours(8))
    .expect("valid timestamp")
    .timestamp();

    let claims = Claims{
        sub:user_email.to_owned(),
        exp:expiration as usize,
    };

    let secret = std::env::var("JWT_SECRET").expect("Jwt secret Missing");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}
fn verify_jwt(token:&str)-> Result<Claims,JwtError>{
    let secret = std::env::var("JWT_SECRET").expect("Could not find the jwt secret");
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())?;
    Ok(token_data.claims)
}
#[post("/register")]
async fn register(info:web::Json<UserRegister>,pool : web::Data<Pool<Postgres>>)-> impl Responder{
    let sql = include_str!("queries/register.sql");
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hash_password =match  argon2.hash_password(&info.password.as_bytes(), &salt){
        Ok(pass)=>pass.to_string(),
        Err(_)=>return HttpResponse::InternalServerError().body("Could not hash password"),
    };

    let result = sqlx::query(sql)
    .bind(&info.name)
    .bind(&info.email)
    .bind(&hash_password)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_)=>HttpResponse::Ok().body("User register successfully"),
        Err(err)=>HttpResponse::InternalServerError().body(format!("Error: {}",err.to_string()))
    }
}
#[post("/login")]
async fn login(pool:web::Data<Pool<Postgres>>,info:web::Json<UserLogin>)-> impl Responder{
    let user =match sqlx::query!("SELECT email,password FROM users WHERE email=$1",info.email)
    .fetch_one(pool.get_ref())
    .await{
        Ok(u)=>u,
        Err(_)=>return HttpResponse::Unauthorized().body("Invalid email or password"),
    };
    let parser_hash=match PasswordHash::new(&user.password){
        Ok(h)=>h,
        Err(_)=>return HttpResponse::InternalServerError().body("Hash parsing error"),
    };
    if Argon2::default().verify_password(&info.password.as_bytes(), &parser_hash).is_ok(){
        match create_jwt(&user.email) {
            Ok(token)=>HttpResponse::Ok().body(token),
            Err(_)=>HttpResponse::InternalServerError().body("Could not create Jwt"),
        }
    }else {
        HttpResponse::Unauthorized().body("Password Incorrect")
    }
}
#[actix_web::main]
async fn main ()-> std::io::Result<()>{
    dotenv::dotenv().ok();
    let pool = match get_db().await {
        Ok(pool)=>pool,
        Err(err)=>{
            eprintln!("Error: {}",err.to_string());
            std::process::exit(1)
        }
    };
    init_tracing();
    info!("Server Starting Now");
    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(TracingLogger::default())
        .service(index)
        .service(register)
        .service(login)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}