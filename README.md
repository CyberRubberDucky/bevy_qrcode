# bevy_qrcode

A Rust project that combines the powerful **Bevy** game engine with **QR code** generation. With this project, you can integrate QR codes directly into your Bevy applications, enabling new ways to interact with games or applications through encoded data.

## Features

- **Bevy Integration**: Seamlessly render QR codes as part of Bevy's game engine.
- **Customizable QR Codes**: Generate QR codes with different configurations, including error correction levels and customization.
- **Flexible Image Rendering**: Embed QR codes in your game scenes or use them as textures for in-game objects.
- **High Performance**: Built using Rust for blazing-fast performance.

## Getting Started

Follow the steps below to set up and start using `bevy_qrcode`:

### Prerequisites

- [Rust](https://www.rust-lang.org/) 1.83.0 or higher
- [Cargo](https://doc.rust-lang.org/cargo/) package manager
- Basic understanding of the [Bevy game engine](https://bevyengine.org/)

### Installation

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
bevy = "0.15.0"
image = "0.25.5"
qrcode = "0.14.1"
```

### Usage

Here’s an example of integrating QR code generation into a Bevy app:

```rust
use bevy::prelude::*;
use qrcode::{QrCode, EcLevel};
use image::Luma;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup() {
    // Example: Generate a QR Code
    let data = "Hello, Bevy!";
    let code = QrCode::with_error_correction_level(data, EcLevel::Q).unwrap();
    let image = code.render::<Luma<u8>>().build();

    // You can now use the generated QR code image in your Bevy app
    println!("QR Code generated for: {}", data);
}
```

## Project Structure

- **`src`**: Contains the core Rust files for the project.
    - `main.rs`: Entry point of the application.
    - Other module files as necessary.

## Contributing

Contributions are welcome! If you have ideas, find bugs, or want to improve this project, feel free to submit a pull request or open an issue.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

- [Bevy](https://bevyengine.org/) for the game engine framework.
- [QRCode Crate](https://crates.io/crates/qrcode) for QR code generation.
- [Image Crate](https://crates.io/crates/image) for rendering utilities.

---

Happy coding with `bevy_qrcode`! 🎉# bevy_qrcode
