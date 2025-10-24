---

# Actix Web Starter Template

A minimal **Actix Web** boilerplate for building Rust backend APIs with **PostgreSQL** integration and **JWT authentication**.

This repo provides a ready-to-use project structure that you can clone and start building your web applications or APIs immediately.

## Features

* Actix Web backend setup
* PostgreSQL connection using `sqlx`
* JWT-based authentication
* Password hashing with `argon2`
* Request logging with `tracing` and `tracing-actix-web`
* Basic `/index`, `/register`, and `/login` endpoints
* Easy to extend for more routes, middleware, or services

## Prerequisites

* Rust (latest stable)
* Cargo
* PostgreSQL (or Supabase connection string)
* `dotenv` for environment variables

## Getting Started

1. **Clone the repo**

```bash
git clone https://github.com/<your-username>/actix-web-starter.git
cd actix-web-starter
```

2. **Create a `.env` file**

Example `.env` file:

```env
DATABASE_URL=your-supabase-or-postgres-link
JWT_SECRET=your_super_secret_key
```

> Note: Users can replace the `DATABASE_URL` with their own Supabase or PostgreSQL link.

3. **Run the project**

```bash
cargo run
```

Server will start at `http://127.0.0.1:8080`.

## API Endpoints

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

## Usage

* Clone and reuse as a **starter template** for your own Actix Web projects.
* Extend with **protected routes** by verifying JWT tokens using `verify_jwt()`.
* Add new **API endpoints**, services, or database models as needed.

## Contributing

Contributions are welcome! Open issues, submit pull requests, or suggest new features.

## License

MIT License

## Acknowledgements

* [Actix Web](https://actix.rs/)
* [sqlx](https://github.com/launchbadge/sqlx)
* [argon2](https://github.com/RustCrypto/password-hashes)
* [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
* [tracing]([https://github.com/to](https://github.com/to)
