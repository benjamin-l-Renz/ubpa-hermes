# Functions

The project contains several functions that serve different purposes:

- **`background_worker()`**  
  Runs every 24 hours and checks whether an application has been in the database for a certain number of days.  
  If so, it automatically updates the application's state.

- **`is_valid_email()`**  
  Validates whether a given string follows the correct email address format.

- **`detect_pdf()`**  
  Detects whether a given byte stream represents a valid PDF file using the `infer` crate.

- **`load_config()`**  
  Loads the configuration file (`config.json`) and returns an `EmailConfig` struct.

- **`send_email()`**  
  Sends an email to a given email address using the `lettre` crate.

Additionally, the project defines various route handler functions that respond to specific HTTP requests.  
A complete list of these route functions can be found in **`main.rs`**.
