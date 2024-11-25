# GitHub Stats RS

GitHub Stats RS is a Rust application that generates GitHub statistics and visualizes them using SVG images.

## Features

- Fetches GitHub statistics using the GitHub API
- Generates SVG images for overview and language statistics
- Configurable template and output folders

## Setup

### Prerequisites

- Rust (latest stable version)
- Cargo (latest stable version)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/github_stats_rs.git
    cd github_stats_rs
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the project:
    ```sh
    cargo run --release
    ```

## Environment Variables

| Environment Variable       | Default Value                | Description                          |
|----------------------------|------------------------------|--------------------------------------|
| `ACCESS_TOKEN`             | None                         | GitHub access token                  |
| `GITHUB_ACTOR`             | None                         | GitHub actor                         |
| `EXCLUDED`                 | `""`                         | Excluded repositories                |
| `EXCLUDED_LANGS`           | `""`                         | Excluded languages                   |
| `EXCLUDE_FORKED_REPOS`     | `true`                       | Exclude forked repositories          |
| `TEMPLATE_FOLDER`          | `resources/templates`        | Folder containing SVG templates      |
| `OUTPUT_FOLDER`            | `resources/generated`        | Folder for generated SVG images      |

## Usage

1. Set the required environment variables:
    ```sh
    export GITHUB_ACCESS_TOKEN=your_github_access_token
    export TEMPLATE_FOLDER=path_to_template_folder
    export OUTPUT_FOLDER=path_to_output_folder
    export LOG_LEVEL=info
    ```

2. Run the application:
    ```sh
    cargo run --release
    ```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.
