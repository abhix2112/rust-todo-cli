# 📝 Rust Todo CLI App

A simple, terminal-based Todo List application built in Rust with features like add, view, edit, delete, and mark as completed. Data is stored securely using base64-encoded JSON files.

---
```bash
✨ Features

- ✅ Add new todos with title, description, status, and priority
- 📋 View all your todos in a structured format
- 🛠️ Edit any todo by ID
- 🗑️ Delete todos you no longer need
- ✅ Mark todos as completed
- 🔒 **Data is securely stored in a base64-encoded file** (`todo.json`)  
- 💾 **Persistent storage** – todos remain saved even after you close the terminal
- 🌈 Emoji-enhanced CLI interface
```
---

## 🚀 Getting Started

### 🔧 Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Install using `rustup`)

### 🔨 Build & Run

```bash
# Clone the repository
git clone https://github.com/abhi2112x/rust-todo-cli.git
cd rust-todo-cli

# Build the project
cargo build

# Run the application
cargo run

```

📂 File Structure
```bash
├── src/
│ └── main.rs # Main application logic
├── todo.json # Base64-encoded todos storage
├── Cargo.toml # Dependencies and metadata
└── README.md # Project documentation
```
📸 Sample Output

📋 Welcome to the Todo List

1. Add Todo
2. View Todos
3. Edit Todo
4. Delete Todo
5. Mark Completed
6. Exit

🛠️ Tech Stack

Rust – Safe and fast systems programming language

Serde – Serialization & deserialization

Chrono – Date & time formatting

Base64 – Encoding for secure local data

📃 License
This project is licensed under the MIT License.

🙌 Acknowledgements
Built with ❤️ by [Abhishek Aggarwal].
