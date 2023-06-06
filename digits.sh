#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 <file_path>"
  exit 1
fi

input_file="$1"
output_file="sanitized.txt"

if [ ! -f "$input_file" ]; then
  echo "File not found: $input_file"
  exit 1
fi

# Remove non-numeric and non-punctuational characters, and collapse repeated punctuations
cat "$input_file" | tr -cs '[:digit:][:punct:]' '\n' | awk '!a[$0]++' > "$output_file"

echo "Sanitized file created: $output_file"
