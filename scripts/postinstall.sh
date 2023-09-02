#!/bin/bash
cp commitiq-cli/target/release/commitiq /usr/local/bin/ciq
echo "Run 'ciq config set <OPENAI_API_KEY>' to start using CommitIQ."
echo "After setting the API key, simply run 'ciq' to generate a commit message based on your git diff."
