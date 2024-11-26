# GitHub Stats RS
![example workflow](https://github.com/callmestech/github-stats-rs/actions/workflows/main.yml/badge.svg)
---
GitHub Stats RS is a Rust application that generates GitHub statistics and visualizes them using SVG images.
<a href="https://github.com/callmestech/github-stats-rs">
<img src="https://github.com/callmestech/github-stats-rs/blob/master/resources/generated/overview.svg#gh-dark-mode-only" />
<img src="https://github.com/callmestech/github-stats-rs/blob/master/resources/generated/languages.svg#gh-dark-mode-only" />
<img src="https://github.com/callmestech/github-stats-rs/blob/master/resources/generated/overview.svg#gh-light-mode-only" />
<img src="https://github.com/callmestech/github-stats-rs/blob/master/resources/generated/languages.svg#gh-light-mode-only" />
</a>
   
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

1. Create a `.env` file with the following content:
    ```sh
    ACCESS_TOKEN=your_access_token
    GITHUB_ACTOR=your_github_login
    ```

2. Run the application:
    ```sh
    cargo run --release | bunyan
    ```


3. To add your statistics to your GitHub Profile README, copy and paste the
   following lines of code into your markdown content. Change the `username`
   value to your GitHub username.
   ```md
   ![](https://raw.githubusercontent.com/username/github-stats-rs/master//resources/generated/overview.svg#gh-dark-mode-only)
   ![](https://raw.githubusercontent.com/username/github-stats-rs/master/resources/generated/overview.svg#gh-light-mode-only)
   ```
   ```md
   ![](https://raw.githubusercontent.com/username/github-stats-rs/master/generated/languages.svg#gh-dark-mode-only)
   ![](https://raw.githubusercontent.com/username/github-stats/master/resources/generated/languages.svg#gh-light-mode-only)
   ```
## Related Projects
- [jstrieb/github-stats](https://github.com/jstrieb/github-stats)

   
