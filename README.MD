# Learning Rust 🦀

So yeah... apparently C++ wasn’t hard enough,  
so now I’ve decided to ruin my peace with Rust.

Because why not fight with the compiler  
and question my life choices _every single day_ like pursuing as Btechhhhh Studentttt?

Memory safety? Ownership? Borrow checker?  
Haan haan, bas yahi cheezein chahiye thi mujhe…

Anyway, welcome to my journey of learning Rust.  
Expect chaos, sarcasm, and maybe… just maybe… some working code...


<!--  -->

# Directory Lister CLI

A simple and elegant **Rust-based Command Line Application** that displays all files and folders in a given directory in a beautifully formatted table with colors.

---

## ✨ Features

- 📁 List files and directories from any path
- 🎨 Styled and colored table output using `tabled` & `owo-colors`
- 🕒 Shows last modified date for each entry
- 📦 Displays file size in bytes
- 🧭 If no path is provided, defaults to current directory
- 🧰 Clean and easy argument parsing with `clap`

---

## 🚀 Usage

### **Run the application**
```sh
cargo run
```

### **Run with a custom directory path**
```sh
cargo run -- /path/to/directory
```

Example:
```sh
cargo run -- C:\Users\Varun\Documents
```

---

## 📦 Build Release Binary
```sh
cargo build --release
```
Binary output will be located in:
```
target/release/
```

---

## 🛠 Technologies Used
| Library | Purpose |
|---------|---------|
| **clap** | CLI argument parser |
| **tabled** | Table formatting & styling |
| **chrono** | Time formatting |
| **owo-colors** | Colored terminal output |
| **std::fs** | Filesystem operations |

---

## 📸 Example Output
```
┌──────────────┬────────┬──────────┬───────────────────────┐
│ Name         │ Type   │ Size     │ Modified              │
├──────────────┼────────┼──────────┼───────────────────────┤
│ src          │ Dir    │ 4096     │ Sun Sep  8 2024       │
│ Cargo.toml   │ File   │ 212      │ Sun Sep  8 2024       │
└──────────────┴────────┴──────────┴───────────────────────┘
```

---

## 🤝 Contributing
Pull requests, improvements, and suggestions are always welcome!

---

## 👤 Author
**Varun Dobhal**  
Rust & Blockchain Developer
