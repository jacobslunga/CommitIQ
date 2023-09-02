# CommitIQ 🚀

## Overview

CommitIQ is a CLI tool designed to simplify and automate your git workflow. It leverages the power of GPT-3 to generate, add, and commit git changes for you.

## Prerequisites

- An OpenAI API key is required. Get one [here](https://beta.openai.com/signup/).

## Installation

You can install CommitIQ using npm, Yarn, or pnpm.

### Using npm

```bash
npm install -g commitiq
```

### Using Yarn

```bash
yarn global add commitiq
```

### Using pnpm

```bash
pnpm add --global commitiq
```

## Commands

- **ciq**: This command automatically generates a commit message, adds files, and commits the changes.
- **ciq config set `<OPENAI_API_KEY>`**: This command sets your OpenAI API key, which is stored in a `.commitiq` file at the root directory.

## Usage Example

Here's a quick example to give you a taste of how CommitIQ works:

```bash
jacobslunga@Schlunkysdator:CommitIQ$ ciq
Generating commit message...

Generated commit message:
Update main.rs, delete multiple files in target/debug/incremental
Do you want to commit with this message? (yes/no/new) yes

Committing...
Runing: git add .
Running: git commit -m "Update main.rs, delete multiple files in target/debug/incremental"

Successfully committed:
Update main.rs, delete multiple files in target/debug/incremental

You can now push your changes to the remote repository🚀.
```

## Limitations

- The tool is in its early stages and may contain bugs.

## Contributing

If you'd like to contribute, please open an issue or submit a pull request. All contributions are welcome!

## Acknowledgments

Special thanks to OpenAI for providing the GPT-3 API that powers the commit message generation.
