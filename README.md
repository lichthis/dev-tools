# Dev Tools

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/leptos-0.6.15-blue.svg)](https://leptos.dev/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A collection of developer tools built with Rust and WebAssembly, featuring a modern web interface and internationalization support.

## Features

- 🌐 Multilingual Support (English & Chinese)
- 🔧 URL Encoder/Decoder
- 📝 Base64 Encoder/Decoder
- 🎯 JSON Formatter
- ⏰ Cron Expression Parser
- 💾 Persistent Language Settings
- ⚡ Fast WebAssembly Performance

## Quick Start

### Prerequisites

- Rust (stable)
- wasm-pack
- Node.js (optional, for development)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/dev-tools.git
cd dev-tools

# Run in development mode
trunk serve
```


## Technology Stack

- **Frontend Framework**: Leptos 0.6.15
- **Internationalization**: rust-i18n 2.0
- **WebAssembly Bindings**: wasm-bindgen, web-sys
- **Styling**: Tailwind CSS

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Leptos](https://leptos.dev/) - The web framework used
- [rust-i18n](https://github.com/longbridgeapp/rust-i18n) - For internationalization support