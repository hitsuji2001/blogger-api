### Description
Blogger website api for my personal project.

### Quick start

Clone this git repo

```bash
git clone git@github.com:hitsuji2001/blogger-api.git
```

then configure `.env-example` file to your liking, don't forget to rename that file to `.env`
Run database and minio server

```bash
docker compose up -d
```

Finally run the web server

```bash
cargo run 
```

### Tech stack used 

- API Framework: [axum](https://github.com/tokio-rs/axum)
- Database: [SurrealDB](https://surrealdb.com/)
- Object Storage: [Minio](https://min.io/)

### References used

- [Crate axum](https://docs.rs/axum/latest/axum/)
- [Crate s3](https://docs.rs/rust-s3/latest/s3/)
- [Crate surrealdb](https://docs.rs/surrealdb/1.0.0-beta.9+20230402/surrealdb/)
- [SurrealDB Documentation](https://surrealdb.com/docs/)
- [JWT authentication in Rust](https://blog.logrocket.com/jwt-authentication-in-rust/)
- [Clean Code with Rust & Axum](https://www.propelauth.com/post/clean-code-with-rust-and-axum)
