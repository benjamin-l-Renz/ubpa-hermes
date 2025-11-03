use actix_files as fs;
use actix_multipart::form::bytes::Bytes;
use actix_multipart::form::text::Text;
use actix_multipart::form::{MultipartForm, tempfile::TempFileConfig};
use actix_web::Error;
use actix_web::web::PayloadConfig;
use actix_web::{App, HttpServer, Responder, web};
use askama::Template;
use askama_web::WebTemplate;

fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();

    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() || domain.is_empty() {
        return false;
    }

    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return false;
    }

    if email.contains(' ') {
        return false;
    }

    true
}

#[derive(Template, WebTemplate)]
#[template(path = "upload.html")]
/// Displays an given error on a html div
struct UploadTemplate<'a> {
    error: &'a str,
}

#[derive(Debug, MultipartForm)]
/// Contains all the variables that are send to the backend from the html form
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<Bytes>,
    #[multipart(rename = "email")]
    email: Option<Text<String>>,
    #[multipart(rename = "checkbox")]
    checkbox: Option<Text<String>>,
}

/// Gets the post request from the html form with all atributes
async fn load(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    if form.files.is_empty() {
        println!("You should send files");
        return Ok(UploadTemplate {
            error: "You must upload a file",
        });
    }

    let count = form.files.len();

    if count > 10 {
        return Ok(UploadTemplate {
            error: "To many files (max: 10)",
        });
    }

    for f in form.files {
        let filename = f.file_name.unwrap();

        println!("working with file: {}", filename);

        if f.data.len() > 10 * 1024 * 1024 {
            println!("to big");
            return Ok(UploadTemplate {
                error: "file is to big",
            });
        }

        let path = format!("./upload/{}", filename);

        if !detect_pdf(&f.data) {
            return Ok(UploadTemplate {
                error: "file must be a pdf",
            });
        } else {
            // add path into the database
            std::fs::write(path, f.data).expect("failed to write data");
            println!("file uploaded");
        }
    }

    match form.email {
        Some(email) => {
            if email.is_empty() {
                return Ok(UploadTemplate {
                    error: "no email suplied",
                });
            } else if is_valid_email(&email) {
                println!("{}", email.as_str())
            } else {
                return Ok(UploadTemplate {
                    error: "email must be a valid format",
                });
            }
        }
        None => {
            return Ok(UploadTemplate {
                error: "no email suplied",
            });
        }
    }

    let checkbox_value = form.checkbox.as_ref().map(|t| t.0 == "on").unwrap_or(false);

    if !checkbox_value {
        return Ok(UploadTemplate {
            error: "you need to agree to the Nutzerbedingungen",
        });
    }

    Ok(UploadTemplate { error: "" })
}

/// Detect if the given file is really a pdf using infer
fn detect_pdf(data: &[u8]) -> bool {
    println!("detecting if pdf");
    if let Some(kind) = infer::get(data) {
        println!("{}", kind);
        kind.mime_type() == "application/pdf"
    } else {
        false
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");

    std::fs::create_dir_all("./upload")?;

    HttpServer::new(|| {
        App::new()
            .app_data(PayloadConfig::new(10 * 1024 * 1024))
            .app_data(TempFileConfig::default().directory("./upload"))
            // Serve everything under /static/
            .service(fs::Files::new("/html/", "./static/html/").prefer_utf8(true))
            .service(fs::Files::new("/js/", "./static/js/").show_files_listing())
            //.service(fs::Files::new("/css/", "./static/css").prefer_utf8(true))
            //.service(fs::Files::new("/static", "./static").prefer_utf8(true))
            // Route for HTMX dynamic response
            // Default route -> serve the main HTML
            .route("/load", web::post().to(load))
            .default_service(
                web::get()
                    .to(|| async { fs::NamedFile::open_async("./static/html/index.html").await }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
