# Actix Web

[Actix Web](https://actix.rs/) is the backend framework used in this project.  
It provides a powerful yet ergonomic way to build web servers and APIs in Rust, making it easy to add functionality and handle requests efficiently.

---

# Routes

Adding routes in Actix Web is straightforward.  
You can define them directly when configuring your application, for example:

```rust
.route("/example", web::get().to(example_function))
```

Each route maps a specific URL and HTTP method to a handler function.
This makes it simple to organize different parts of your web application in a clean and maintainable way.
