<div align="center">

# 🦀 Rust + ActixWeb — API Server App Boilerplate

[![Rust](https://img.shields.io/badge/Rust-orange?style=for-the-badge&logo=rust&logoColor=white)](https://rust-lang.org/)
[![ActixWeb](https://img.shields.io/badge/ActixWeb-black?style=for-the-badge&logo=actix&logoColor=white)](https://actix.rs/)
[![JWT](https://img.shields.io/badge/Json%20Web%20Token-purple?style=for-the-badge&logo=JSON%20web%20tokens)](https://www.jwt.io/)


</div>

## Introduction

🦀 **Rust is the future of low-level programming.** It delivers fast, safe, and highly performant applications, making it an excellent choice for server-side development. Setting up a Rust server can be more complex due to the required integrations and configurations — but the result is a powerful, reliable, and lightning-fast server.

---

## Features

- **ActixWeb:** A powerful, pragmatic, and extremely fast web framework for Rust
- **Async Processes:** Powered by **Tokio** crate for asynchronous programming.
- **JWT Manager:** A JSON Web Token manager to handle token processes.
- **Middleware:** An auth guard to filter the public and private routes using JWT.
- **Encryption:** A Strong layer of security by Encrypt/Decrypt JWT Payloads using AES Encryption Algorithm.
- **Cors:** Add a layer of security by enabling server-app cors using actix-cors.
- **Accepted Header:** Add a layer of security by specifyng the only accepted headers.
- **Testing:** Multiple unit and integration tests for many methods.

---

## Usage

### Clone the project to your machine with git:

```
git clone https://github.com/hamzaelmarjani/rust-actixweb-boilerplate.git
```

### Run Server App:

```
cd rust-actixweb-boilerplate && cargo run

===========================================================================================
 _______ _____  _____  ______  _________        _      _______ _____
|_   __ |_   _||_   _.' ____ \|  _   _  |      / \    |_   __ |_   _|
  | |__) || |    | | | (___ \_|_/ | | \_|     / _ \     | |__) || |
  |  __ / | '    ' |  _.____`.    | |        / ___ \    |  ___/ | |
 _| |  \ \_\ \__/ /  | \____) |  _| |_     _/ /   \ \_ _| |_   _| |_
|____| |___|`.__.'    \______.' |_____|   |____| |____|_____| |_____|

      ______  ________ _______ ____   ____ ________ _______
    .' ____ \|_   __  |_   __ |_  _| |_  _|_   __  |_   __ \
    | (___ \_| | |_ \_| | |__) |\ \   / /   | |_ \_| | |__) |
     _.____`.  |  _| _  |  __ /  \ \ / /    |  _| _  |  __ /
    | \____) |_| |__/ |_| |  \ \_ \ ' /    _| |__/ |_| |  \ \_
     \______.|________|____| |___| \_/    |________|____| |___|

===========================================================================================
🦀 Server running on: http://localhost:8080 or http://127.0.0.1:8080 or http://0.0.0.0:8080
🔌 Press Cmd/Ctrl+C to stop and quit the server.
💪 Keep building amazing things — you’ve got this!
===========================================================================================
```

### Test the app on your browser, open: [http://localhost:8080/health](http://localhost:8080/health)

---

## Project Structure:

The project is built using the MVC architecture, it was designed to be scalable VERTICALLY, following the "Monolithic" rules, but is ready to be separated to "Micro Services." The project structure is as follows:

- **src/controllers:** Controllers are responsible for handling requests and responses.
- **src/views:** Views are responsible for displaying data to the user.
- **src/middleware, src/jwt, src/mails, src/constants, src/structs, src/utils, src/test:** Models to handle the business logic.

### Controllers:

All controllers are located in **src/controllers**, and organized as the following:

- **crud**: Controllers for CRUD operations (create, read, update, delete).
- **user**: Controllers for user authentication and registration (sign-in, sign-up, lifecycle and email verification).
- **newsletter:** Controllers for newsletter subscription and mailing.

### Views:

All views are located in **src/views**, and organized as the following:

- **user:** Index view if you want to response with an HTML page or redirect to another page.
- **newsletter:** Views for newsletter subscription and mailing.

### Middleware, JWT, Mails, Constants, Structs, Utils, Tests:

All models are located in **src/middleware, src/jwt, src/mails, src/constants, src/structs, src/utils, src/test**, and organized as the following:

- **jwt (JWT Manager):** Includes the JSON Web Token manager to handle token processes as well as the encryption layer.
- **middleware:** Includes the auth guard to filter the public and private routes using JWT Manager.
- **test:** Includes the unit and integration tests for many methods.
- **mails:** Includes Mail Manager to prepare and send emails.
- **constants:** Inludes the constant app data which is not changed frequently.
- **structs:** Includes the structs of all services across the app (types), all in one place.
- **utils:** Includes the helper functions to be used across the app.

---

## App Lifecycle:

The server-running process passed by those steps:

1. Load the environment variables from the `.env` file.
2. Call all the initializers, like: databases url, redis, aws, gcp, azure, etc.
3. Start the server on port 8080.

The server request always passed by those steps:

1. Middleware Auth Guard filter check if the request includes a public path or private one:
   1. If public, the middleware will call the next handler function.
   2. If private, The middleware will extract the access_token from the Authorization Bearer header and validate it with the JWT Manager:
      1. If the token is valid, and its payload includes the user info, such as user_id, email, etc. The middleware will call the next handler function.
      2. Otherwise, the middleware will return a 401 Unauthorized error.
2. Call the handler function.
3. Return the response.

---

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation or examples

Before contributing, please ensure your code follows Rust + ActixWeb conventions.

## 📬 Contact & Hire Me

🚀 Want to add more advanced features to this project (SQL/NoSQL database, caching, queue system like "Kafka, Pub/Sub," etc.) I’ve got you covered! You can hire me.

**Company or Startup?** I can work **full-time** or **part-time**, 👉 **Remote** or **On-site**.

💌 Reach me at: **hamzaelmarjani@gmail.com**

✨ Thank you!

## Support

If you like this project, consider supporting me on Patreon 💖

[![patreon](https://img.shields.io/badge/Support-Open_Source-black?style=for-the-badge&logo=Patreon&logoColor=white)](https://www.patreon.com/elmarjanihamza/gift)

❤️ Thanks for reading, Happy Coding 💻
