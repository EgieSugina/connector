# Connector

Connector is a simple Rust application that establishes an SSH connection and sets up a SOCKS5 proxy. This allows you to route your internet traffic through a secure SSH tunnel.

## Features

- Establishes an SSH connection to a specified server.
- Sets up a SOCKS5 proxy on `localhost:1080`.
- Provides instructions for configuring your browser to use the proxy.

## Prerequisites

- Rust and Cargo installed on your system.
- SSH client installed and accessible from your command line.
- A valid SSH account on the server you wish to connect to.

## Installation

1. **Clone the repository**:

   ```bash
   git clone https://github.com/yourusername/connector.git
   cd connector
   ```

2. **Build the project**:

   ```bash
   cargo build --release
   ```

3. **Run the executable**:

   ```bash
   ./target/release/connector user@server.com
   ```

   Replace `user@server.com` with your actual SSH username and server address.

## Usage

- **Command**: `connector user@server.com`
- **Example**: `connector alice@example.com`

This command will establish an SSH connection and set up a SOCKS5 proxy on `localhost:1080`.

## Configuring Your Browser

To use the SOCKS5 proxy, configure your browser to use a SOCKS5 proxy at `localhost` on port `1080`. Here are the steps for some popular browsers:

### Firefox

1. Open Firefox and go to `Preferences`.
2. Scroll down to `Network Settings` and click `Settings`.
3. Select `Manual proxy configuration`.
4. Enter `localhost` and `1080` in the `SOCKS Host` field.
5. Ensure `SOCKS v5` is selected.
6. Click `OK` to save the settings.

### Chrome

For Chrome, you can use an extension like [SwitchyOmega](https://github.com/FelisCatus/SwitchyOmega) to manage proxy settings.

1. Install SwitchyOmega from the Chrome Web Store.
2. Create a new profile and set the protocol to `SOCKS5`.
3. Enter `localhost` and `1080` as the host and port.
4. Apply the settings and switch to the new profile.

## Troubleshooting

- Ensure your SSH credentials are correct.
- Verify that the SSH server is accessible and running.
- Check your firewall settings to ensure they allow outgoing connections on port 1080.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details. 