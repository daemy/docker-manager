# docker-manager

docker-manager is a Rust-based utility designed to streamline the development and management of Docker containers. It automates the process of rebuilding and restarting Docker containers upon file changes, ensuring that your development workflow remains efficient and uninterrupted. With docker-manager, you can easily monitor and handle your containers, view real-time logs, and manage environment configurations with ease.

Primarily made for local development, with future plans to implement full CI/CD management toolkit for orchestrating Docker containers.

Key Features:
- Automatic Hot-Reloading: Automatically rebuild and restart containers when changes are detected in your project files.
- Real-Time Logging: Seamlessly view and follow logs from your Docker containers to monitor their status and troubleshoot issues.
- Configurable via YAML: Customize paths, container names, and other settings through a user-friendly YAML configuration file.
- Cross-Platform Support: Available for Linux, Windows, and macOS, providing a consistent experience across different environments.
- Efficient Development Workflow: Reduces the manual effort involved in container management, allowing you to focus on development.


# version

This is a pre-0.0.1 version, still in initial planning phase. Only basic functionality (proof of concept) is implemented.


# run/install

Currently there are only linux binaries available, to install the current version system-wide, you need to run in your terminal:

1. ./install.sh

Afterwards, just go to any repo that has a proper docker-compose.yaml file and just run:

2. docker-manager [container-name]
