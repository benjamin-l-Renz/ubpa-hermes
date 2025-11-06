# Setup

To get started, install Rust by following the official instructions at  
https://rust-lang.org/tools/install/

Then, Clone the repository:

```bash
git clone https://github.com/uxitra/ubpa-hermes.git
```

Most folders and files are created automatically when you first build or run the project.
However, there is one important exception — the configuration file.

The configuration file (config.json) contains critical information such as the sender email address and password, which are required for sending emails from Rust.

An example configuration can be found in the project root.
You can copy it and adjust the values to match your setup.

Example config.json:

```json
{
  "email": "your-email@example.com",
  "password": "password",
  "subject": "Test",
  "email_content": "Hello test from rust"
}
```
