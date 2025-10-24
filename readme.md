# Actix Web Starter Template 🚀

A minimal **Actix Web** boilerplate for building Rust backend APIs with **PostgreSQL** (or Supabase) integration and **JWT authentication**.

This repo provides a ready-to-use project structure that you can clone and start building your web applications or APIs immediately.

---

## Features ✅

* Actix Web backend setup
* PostgreSQL connection using `sqlx`
* JWT-based authentication
* Password hashing with `argon2`
* Request logging with `tracing` and `tracing-actix-web`
* Basic `/index`, `/register`, and `/login` endpoints
* Easy to extend for protected routes, middleware, or additional services
* Ready for Docker (optional)
* Example `.env.example` for quick setup

---

## Badges

![Rust](https://img.shields.io/badge/rust-1.71+-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

---

## Prerequisites ⚙️

* Rust (latest stable)
* Cargo
* PostgreSQL or Supabase database
* `dotenv` for environment variables

---

## Getting Started 🏁

1. **Clone the repo**

```bash
git clone https://github.com/DRAKEN_1974/actix-web-starter.git
cd actix-web-starter
```

2. **Create a `.env` file**

Copy the example file:

```bash
cp .env.example .env
```

Fill in your credentials:

```env
DATABASE_URL=your-supabase-or-postgres-link
JWT_SECRET=your_super_secret_key
```

> Note: Users can replace the `DATABASE_URL` with their own Supabase or PostgreSQL link.

3. **(Optional) Docker Setup**

Run PostgreSQL via Docker:

```bash
docker-compose up -d
```

4. **Run the project**

```bash
cargo run
```

Server will start at `http://127.0.0.1:8080`.

---

## API Endpoints 🛠️

### GET `/index`

* Returns: `"This is the main page"`

### POST `/register`

* Body (JSON):

```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "yourpassword"
}
```

* Registers a new user with hashed password.

### POST `/login`

* Body (JSON):

```json
{
  "email": "john@example.com",
  "password": "yourpassword"
}
```

* Returns a JWT token if credentials are correct.

### Example Protected Route `/protected`

* Add this in your `main.rs` to test JWT verification:

```rust
#[get("/protected")]
async fn protected(req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get("Authorization").unwrap().to_str().unwrap();
    if auth_header.starts_with("Bearer ") {
        let token = &auth_header[7..];
        match verify_jwt(token) {
            Ok(claims) => HttpResponse::Ok().body(format!("Hello, {}", claims.sub)),
            Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
        }
    } else {
        HttpResponse::Unauthorized().body("Missing token")
    }
}
```

> This demonstrates how to create protected endpoints using JWT.

---

## Usage 💡

* Clone and reuse as a **starter template** for your own Actix Web projects.
* Extend with **protected routes** by verifying JWT tokens using `verify_jwt()`.
* Add new **API endpoints**, services, or database models as needed.
* Ideal for **learning Actix Web, Rust, and JWT integration**.

---

## Contributing 🤝

Contributions are welcome! Open issues, submit pull requests, or suggest new features.

---

## License 📄

MIT License

---

## Acknowledgements 🙏

* [Actix Web](https://actix.rs/) – The web framework used
* [sqlx](https://github.com/launchbadge/sqlx) – Async SQL toolkit
* [argon2](https://github.com/RustCrypto/password-hashes) – Password hashing
* [jsonwebtoken](https://github.com/Keats/jsonwebtoken) – JWT handling
* [tracing](https://github.com/tokio-rs/tracing) – S
