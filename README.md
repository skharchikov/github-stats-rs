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

<!--
##### WIP: Animated contribution heatmap

---
<a href="https://github.com/callmestech/github-stats-rs">
<img src="https://github.com/callmestech/github-stats-rs/blob/master/resources/generated/contribution_grid.svg#gh-dark-mode-only" />
<img src="https://github.com/callmestech/github-stats-rs/blob/master/resources/generated/contribution_grid.svg#gh-light-mode-only" />
</a>
-->

## Features

- Fetches GitHub statistics using the GitHub API
- Generates SVG images for overview and language statistics
- Configurable template and output folders

## Setup

### Prerequisites

- Rust (latest stable version)
- Cargo (latest stable version)

### Installation

1. Create a personal access token (not the default GitHub Actions token)

2. Create a copy of this repository by clicking [here](https://github.com/callmestech/github-stats-rs/generate)

3. In the repository settings, navigate to the `Secrets` tab and add the following secrets:
    - `ACCESS_TOKEN`: GitHub access token

4. Go to the `Actions` tab and run the workflow

5. Take note of the generated SVG images in the output folder

6. To add your statistics to your GitHub Profile README, copy and paste the
   following lines of code into your markdown content. Change the `username`
   value to your GitHub username.

   ```md
   ![](https://raw.githubusercontent.com/username/github-stats-rs/master/resources/generated/overview.svg#gh-dark-mode-only)
   ![](https://raw.githubusercontent.com/username/github-stats-rs/master/resources/generated/overview.svg#gh-light-mode-only)
   ```

   ```md
   ![](https://raw.githubusercontent.com/username/github-stats-rs/master/resources/generated/languages.svg#gh-dark-mode-only)
   ![](https://raw.githubusercontent.com/username/github-stats-rs/master/resources/generated/languages.svg#gh-light-mode-only)
   ```

## Environment Variables

| Environment Variable       | Default Value                | Description                          |
|----------------------------|------------------------------|--------------------------------------|
| `ACCESS_TOKEN`             | None                         | GitHub access token                  |
| `GITHUB_ACTOR`             | None                         | GitHub actor                         |
| `EXCLUDED`                 | `""`                         | Excluded repositories                |
| `EXCLUDED_LANGS`           | `""`                         | Excluded languages                   |
| `EXCLUDE_FORKED_REPOS`     | `true`                       | Exclude forked repositories          |
| `LANGUAGES_LIMIT`     | 10                       | Amount of languages represented on svg          |
| `TEMPLATE_FOLDER`          | `resources/templates`        | Folder containing SVG templates      |
| `OUTPUT_FOLDER`            | `resources/generated`        | Folder for generated SVG images      |

## Local Development

1. Create a `.env` file with the following content:

    ```sh
    ACCESS_TOKEN=your_access_token
    GITHUB_ACTOR=your_github_login
    ```

2. Run the application:

    ```sh
    cargo run --release | bunyan
    ```

3. Open the generated SVG images in the output folder.

## Related Projects

- [jstrieb/github-stats](https://github.com/jstrieb/github-stats)
