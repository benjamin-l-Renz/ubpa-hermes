# Askama Templates

[Askama](https://github.com/askama-rs/askama) is a Rust HTML template engine that allows you to embed Rust expressions directly in your HTML code.  
It provides a type-safe, compile-time checked way to generate dynamic web pages, similar to Jinja2 in Python or Tera in Rust.

---

## Creating an Askama Template

To create a new Askama template, you first define an HTML template file (for example, `template.html`) inside the `templates/` folder.  
Then, you register it in Rust by defining a struct and deriving the necessary traits:

```rust
use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "template.html")]
/// Askama template struct representing the `template.html` file.
pub struct ExampleTemplate;

```

## Using Templates in Actix-Web

When you also derive the WebTemplate trait, your template can be returned directly from an Actix handler function as a valid HTTP response:

```rust
async fn example() -> Result<impl actix_web::Responder, actix_web::Error> {
    Ok(ExampleTemplate {})
}

```

Askama automatically recreates the templates at compile time so there a nearly no runtime errors
