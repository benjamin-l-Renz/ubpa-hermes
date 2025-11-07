# Structure

```
my_project/
│
├─ src/
├─ docs/
├─ static/
├─ templates/
└─ data/
```

## General Overview

The project is organized into several key folders:

- src/ — contains the main Rust source code.

- docs/ — holds all project documentation.

- static/ — contains static assets such as HTML and CSS files.

- templates/ — includes Askama HTML templates used by the Rust backend.

- data/ — stores the SQLite database and related files.

The static/ folder contains the base HTML file (index.html), which serves as a container for all Askama templates.
As a result, most of the HTML content is dynamically generated through Askama rather than written directly in plain HTML.

Inside the src/ directory, there’s an additional templates/ module that defines the Askama template structs, keeping template logic separated from the core application logic.

## Why is this also a node project ?

The project is a cargo and a npm project at the same time this is needed for installing tailwindcss and the tailwindcss cli so the project doesn't need to ship the precompiled tailwindcss binary's

If tailwindcss and tailwindcss cli aren't automatically installed it is recommended trying to run:

```bash
npm install
```
