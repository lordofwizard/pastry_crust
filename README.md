
# Pastry_Crust- Documentation
![Image](Pastry_crust.png)

Pastry_crust is a minimal pastebin-type application inspired by Rust's sweetness, allowing users to quickly share code snippets and text content.

## Table of Contents

- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
  - [Running the Application](#running-the-application)
  - [Accessing the Paste Webpage](#accessing-the-paste-webpage)
  - [Submitting a Paste](#submitting-a-paste)
- [Contributing](#contributing)
- [Used Technologies and Dependencies](#used-technologies-and-dependencies)
- [License](#license)

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- Rust programming language (Recommended version: [Install Rust](https://www.rust-lang.org/tools/install))
- Cargo package manager (Usually comes with Rust installation)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/lordofwizard/pastry_crust.git
   ```

2. Change into the project directory:

   ```bash
   cd pastry_crust
   ```

3. Build the project:

   ```bash
   cargo build
   ```

## Usage

### Running the Application

Run the application using Cargo:

```bash
cargo run
```

### Accessing the Paste Webpage

Open your web browser and navigate to:

```
http://localhost:8080
```

You will be greeted with the Pastry_Crust webpage.

### Submitting a Paste

1. Enter your code or text content in the textarea.
2. Click the "Submit" button.
3. You will be redirected to a page displaying your paste token.

## Contributing

We welcome contributions! If you'd like to contribute to the project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature/bugfix.
3. Make your changes and commit them.
4. Push your changes to your forked repository.
5. Create a pull request.

## Used Technologies and Dependencies

This project is built using the [Rust programming language](https://www.rust-lang.org/). It utilizes the following Rust crates:
- [actix-web](https://crates.io/crates/actix-web) 
- [rusqlite](https://crates.io/crates/rusqlite)
- [rand](https://crates.io/crates/rand)
- [actix-files](https://crates.io/crates/actix-files)
- [serde](https://crates.io/crates/serde)

The project also incorporates [Tailwind CSS](https://tailwindcss.com/) for styling.

## License

This project is licensed under the [MIT License](LICENSE).

---
