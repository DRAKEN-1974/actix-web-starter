use std::env;

use actix_web::{ middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::{postgres::PgPoolOptions,  Pool, Postgres};
use serde::Deserialize;

#[derive(Deserialize)]
struct UserRegister{
    name:String,
    email:String,
    password:String,
}
#[derive(Deserialize)]
struct Login{
    email:String,
    password:String,
}

#[post("/register")]
async fn register(pool:web::Data<Pool<Postgres>>,info:web::Json<UserRegister>)-> impl Responder{
    let sql = include_str!("queries/register.sql");
    let salt = SaltString::generate(&mut OsRng);
    let argon2=Argon2::default();
    let hashedpass = match argon2.hash_password(info.password.as_bytes(), &salt){
        Ok(hash)=>hash.to_string(),
        Err(_)=>{return HttpResponse::InternalServerError().body("Error could not hash the password")}
    };
    let result = sqlx::query(sql)
    .bind(&info.name)
    .bind(&info.email)
    .bind(&hashedpass)
    .execute(pool.get_ref())
    .await;

    match result{
        Ok(_)=>HttpResponse::Ok().body("User registered successfully"),
        Err(err)=>HttpResponse::InternalServerError().body(format!("There is some error : {}",err.to_string()))
    }
}
#[post("/login")]
async fn login(pool:web::Data<Pool<Postgres>>,info:web::Json<Login>)-> impl Responder{
    let user = sqlx::query!("SELECT email,password FROM users WHERE email= $1",
    info.email
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|_|HttpResponse::InternalServerError().body("User not found"));

    let paresed_hash = PasswordHash::new(&user.password).map_err(|_|HttpResponse::InternalServerError().body("Hash Parssing error"));
    if Argon2::default().verify_password(&info.password.as_bytes(), &paresed_hash)
    .is_ok()
    {
        HttpResponse::Ok().body("Login Successful")
    }else{
        HttpResponse::Unauthorized().body("Invalid email or password")
    }
}
async fn get_db()-> Pool<Postgres>{
    let database_url = env::var("DATABASE_URL").expect("Cant find the database url in the env file ");
    let pool : Pool<Postgres>= PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await.expect("Cant connect to the database ");
    return pool;
}
#[actix_web::main]
async fn main ()-> std::io::Result<()>{
    dotenv::dotenv().ok();
    let pool = get_db().await;
    if std::env::var_os("RUST_LOG").is_none(){
        unsafe{
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    env_logger::init();
    HttpServer::new(move||{
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .wrap(Logger::default())
        .service(register)
        .service(login)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}