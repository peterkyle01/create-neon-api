# 🚀 Create Neon API

[![Crates.io](https://img.shields.io/crates/v/create-neon-api.svg)](https://crates.io/crates/create-neon-api)
[![Downloads](https://img.shields.io/crates/d/create-neon-api.svg)](https://crates.io/crates/create-neon-api)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A beautiful CLI tool to bootstrap clean, production-ready Rust backend projects with JWT authentication and Neon PostgreSQL database integration.

**📦 Available on [crates.io/crates/create-neon-api](https://crates.io/crates/create-neon-api)!**

## ✨ Features

- 🎨 **Beautiful CLI Interface** - Colorful and interactive prompts
- 📦 **Template-based** - Clones from a pre-configured Rust backend template
- 🔧 **Auto-configuration** - Automatically updates project name in `Cargo.toml`
- 🏗️ **Ready to Build** - Runs initial `cargo build` to fetch dependencies
- ✅ **Validation** - Ensures project names follow Cargo package naming conventions
- 🔐 **Production Ready** - Includes JWT authentication and PostgreSQL integration

## 📋 Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Git](https://git-scm.com/)
- Internet connection for cloning the template

## 🛠️ Installation

### Option 1: Install from Crates.io (Recommended)

```bash
cargo install create-neon-api
```

### Option 2: Install from Source

```bash
git clone https://github.com/peterkyle01/create-neon-api.git
cd create-neon-api
cargo install --path .
```

## 🚀 Usage

After installation, simply run the tool from anywhere in your terminal:

```bash
create-neon-api
```

The tool will start with a beautiful interactive interface:

```
🚀 Rust Backend Project Generator
═══════════════════════════════════
📝 Enter your project name: my-awesome-api
🚀 Creating project 'my-awesome-api'...
✅ Template cloned successfully!
📝 Updated Cargo.toml with project name
📦 Running initial `cargo build`...
🎉 Project created successfully!
Next steps: `cd my-awesome-api`, configure your `.env` file, and run `cargo run`.
```

### Quick Start

1. **Install the tool:**

   ```bash
   cargo install create-neon-api
   ```

2. **Create a new project:**

   ```bash
   create-neon-api
   ```

3. **Follow the prompts** and enter your project name

4. **Start developing:**
   ```bash
   cd your-project-name
   cp .env.example .env
   # Edit .env with your configuration
   cargo run
   ```

### Project Name Validation

The tool validates project names to ensure they follow Cargo package naming conventions:

- ✅ Lowercase letters, numbers, hyphens, and underscores only
- ✅ Cannot start or end with hyphens
- ✅ Cannot be empty

Examples:

- ✅ `my-api-server`
- ✅ `user_service`
- ✅ `backend2024`
- ❌ `My-API` (uppercase letters)
- ❌ `-invalid-start` (starts with hyphen)
- ❌ `invalid@name` (special characters)

## 📁 What You Get

The generated project includes:

- 🔐 **JWT Authentication** - Ready-to-use authentication system
- 🗄️ **PostgreSQL Integration** - With Neon database support
- 🌐 **RESTful API Structure** - Well-organized endpoints
- ⚙️ **Environment Configuration** - `.env` file support
- 🧪 **Testing Setup** - Unit and integration tests
- 📝 **Documentation** - Comprehensive API documentation
- 🚀 **Production Ready** - Optimized for deployment

## 🔧 Next Steps After Project Creation

1. **Navigate to your project:**

   ```bash
   cd your-project-name
   ```

2. **Configure environment variables:**

   ```bash
   cp .env.example .env
   # Edit .env with your database credentials and JWT secret
   ```

3. **Run the development server:**

   ```bash
   cargo run
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

## 🎯 Template Repository

This tool clones from the [Rust Backend Template](https://github.com/peterkyle01/rust-backend-template) repository, which includes:

- **Axum** web framework
- **SQLx** for database operations
- **JWT** for authentication
- **Serde** for serialization
- **Tokio** async runtime
- **Configuration management**
- **Error handling**
- **Logging**

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Peter Mwangi**

- Email: kylepeterkoine4@gmail.com
- GitHub: [@peterkyle01](https://github.com/peterkyle01)

## 🙏 Acknowledgments

- Thanks to the Rust community for the amazing ecosystem
- Inspired by create-react-app and similar bootstrapping tools
- Built with love for the Rust backend development community

## 📊 Version History

- **v0.1.1** - Latest release
  - Published and available on crates.io! 🎉
  - Updated documentation and examples
- **v0.1.0** - Initial release
  - Interactive CLI interface
  - Template cloning and configuration
  - Project name validation
  - Automatic dependency building

---

**Happy coding! 🦀✨**
