# CSV to JSON Converter

A command-line tool written in Rust that converts CSV files to JSON format.

## Features

- Convert CSV files to JSON
- Support for flat and nested field structures
- Nested fields using dot notation (e.g., `user.name`)
- Pretty-printed JSON output

## Installation

Requires Rust 1.70 or higher.

```bash
git clone <repository-url>
cd csv_to_json
cargo build --release
```

## Usage

```bash
cargo run -- --input data.csv
```

## Examples

### Flat Structure

Input:
```csv
name,age,email
Alice,25,alice@example.com
```

Output:
```json
[{"name": "Alice", "age": "25", "email": "alice@example.com"}]
```

### Nested Structure

Input:
```csv
id,user.name,user.email
1,Alice,alice@example.com
```

Output:
```json
[{"id": "1", "user": {"name": "Alice", "email": "alice@example.com"}}]
```

## How It Works

Use dot notation in CSV headers to create nested JSON:
- `name` → `{"name": "value"}`
- `user.name` → `{"user": {"name": "value"}}`
- `user.contact.email` → `{"user": {"contact": {"email": "value"}}}`