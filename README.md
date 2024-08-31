# RFirewall

**RFirewall** is a lean, modern firewall designed to offer top-tier security with an exceptional user experience. Built with performance and usability in mind, RFirewall aims to provide seamless protection for your Linux environment without compromising on speed or ease of use.

## Planned Features

- **Minimal Overhead**: RFirewall will operate efficiently, ensuring your system's performance is not compromised.
- **Intuitive Interface**: A user-friendly interface is planned to make configuring and managing your firewall straightforward, even for those new to networking.
- **Advanced Threat Detection**: Future updates will include state-of-the-art algorithms to detect and block potential threats before they can compromise your system.
- **Real-Time Monitoring**: A feature for live updates on your network traffic and potential security events is in the works.
- **Customizable Rules**: Plans include the ability to easily create and manage custom firewall rules tailored to your specific needs.
- **Seamless Integration**: RFirewall will integrate smoothly with your existing Linux, ensuring compatibility and ease of deployment.

## Project Description
RFirewall is a cutting-edge, user-centric firewall solution for Linux systems, currently in the planning and development stages. The project focuses on being lean and efficient while delivering a superior user experience. RFirewall aims to make managing your firewall easy and effective, blending performance, security, and usability into one modern package.

## Tech Stack

RFirewall is built using the Rust programming language, chosen for its performance, safety, and modern features. The project relies on several key dependencies to provide a robust and efficient firewall solution:

- **[chrono](https://crates.io/crates/chrono) (v0.4.38)**: A date and time library for Rust, used for handling timestamps and time-based operations.
- **[clap](https://crates.io/crates/clap) (v4.5.16)**: A command-line argument parser with a derive macro, making it easy to build a user-friendly CLI for RFirewall.
- **[ctrlc](https://crates.io/crates/ctrlc) (v3.4.5)**: Allows the application to handle Ctrl+C signals, enabling graceful shutdowns and cleanup.
- **[dirs-next](https://crates.io/crates/dirs-next) (v2.0.0)**: Provides cross-platform support for finding user directories, making configuration and data storage consistent.
- **[pnet](https://crates.io/crates/pnet) (v0.35.0)**: A cross-platform packet manipulation library, essential for capturing and filtering network traffic.
- **[serde](https://crates.io/crates/serde) (v1.0.209)** and **[serde_derive](https://crates.io/crates/serde_derive) (v1.0.209)**: Used for serializing and deserializing data, crucial for configuration management and data storage.
- **[serde_json](https://crates.io/crates/serde_json) (v1.0.127)**: A JSON library for Rust, used to handle configuration files and data interchange.


## Installation

Installation instructions will be provided as the project progresses.

## Initial Usage (Under Development)

As RFirewall is still under development, the initial functionality is limited but expanding. Here's how you can use the current features:

### Start the Firewall

To start the firewall:

```bash
sudo rfirewall start
```

### Stop the Firewall

To stop the firewall:

```bash
sudo rfirewall stop
```

### Add a Custom Rule

You can add custom rules to allow or deny traffic on specific ports and IPs. For example, to allow TCP traffic on port 80 from a specific IP:

```bash
sudo rfirewall add-rule --allow --port=80 --ip=192.168.1.1 --protocol=tcp
```

To deny UDP traffic on port 53 from a specific IP:
```bash
sudo rfirewall add-rule --deny --port=53 --ip=192.168.1.2 --protocol=udp
```

## Contributing

Soon.

## License

RFirewall will be licensed under the MIT License. See the [LICENSE](LICENSE) file for more information once available.