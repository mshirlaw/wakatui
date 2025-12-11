# WakaTUI

<div align="center">
  <img src="img/ascii-art-text.png" alt="WakaTUI" />
  <p>
    <strong>A Terminal User Interface for WakaTime</strong>
  </p>
  <p>
    <a href="https://github.com/mshirlaw/wakatui/actions"><img src="https://github.com/mshirlaw/wakatui/workflows/CI/badge.svg" alt="CI"/></a>
    <a href="https://github.com/mshirlaw/wakatui/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"/></a>
  </p>
</div>

WakaTUI is a terminal user interface for viewing your WakaTime statistics. It provides a quick and easy way to see your coding activity without leaving the terminal.

## Features

- View your coding statistics for different time ranges (e.g., today, last 7 days, etc.).
- See a breakdown of your time spent by language, project, and editor.
- Interactive charts to visualize your coding activity.
- Securely authenticate with your WakaTime account.

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- A WakaTime account
- Your WakaTime API key (get it from [wakatime.com/settings/api-key](https://wakatime.com/settings/api-key))

### Installation

1.  Clone the repository:
    ```sh
    git clone https://github.com/mshirlaw/wakatui.git
    ```
2.  Navigate to the project directory:
    ```sh
    cd wakatui
    ```
3.  Build the project:
    ```sh
    cargo build --release
    ```
4.  Run the application:
    ```sh
    ./target/release/wakatui
    ```
5.  Provide your WakaTime API key (get it from [wakatime.com/settings/api-key](https://wakatime.com/settings/api-key)):

    **Option 1:** If you already use WakaTime, your `~/.wakatime.cfg` file will be auto-detected.

    **Option 2:** Set the `WAKATIME_API_KEY` environment variable:

    ```sh
    export WAKATIME_API_KEY=<your-api-key>
    ./target/release/wakatui
    ```

    **Option 3:** On first launch, enter your API key in the TUI input screen.
    It will be securely stored in your system keychain for future use.

## Usage

Once the application is running with a valid API key, you can view your WakaTime statistics.

### Keyboard Shortcuts

| Key          | Action                     |
| ------------ | -------------------------- |
| `q` or `Esc` | Quit the application       |
| `Ctrl+C`     | Force quit the application |

### Authentication

WakaTUI uses API key authentication (the same method as the official WakaTime desktop app). You have three options for providing your API key (checked in this order):

**Option 1: WakaTime Config File (Recommended for existing users)**

- If you already use WakaTime, WakaTUI will automatically read your API key from `~/.wakatime.cfg`
- No additional setup needed - just run the app!
- This is the same config file used by the official WakaTime plugins

**Option 2: Environment Variable (Temporary)**

- Set the `WAKATIME_API_KEY` environment variable before running the app
- The key is only available for that session and won't be persisted
- Useful for CI/CD or temporary usage

**Option 3: Keychain Storage (Persistent)**

1. Launch WakaTUI - you'll see an API key input screen
2. Visit [wakatime.com/settings/api-key](https://wakatime.com/settings/api-key) to get your API key
3. Type or paste your API key into the input field (it will be masked with asterisks)
4. Press Enter to validate and save to your system keychain
5. The key will be automatically loaded on future launches

**Security Note:** When stored in the keychain, your API key is encrypted by your operating system and never committed to any repository.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
