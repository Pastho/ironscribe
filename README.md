# IronScribe

A flexible and extensible logging framework for Rust applications with support for multiple destinations and colored console output.

## Overview

IronScribe is a modern Rust logging framework that provides:
- **Multiple log destinations** via pluggable architecture (console, MongoDB, PostgreSQL)
- **Colored console output** with semantic colors for different log levels
- **Grouped logging** via log units for related log messages
- **Database persistence** with configurable table/collection names
- **Feature-based configuration** using Cargo features
- **Async/await support** throughout the API

## Features

### Core Functionality
- 📝 **Log Units**: Group related log messages under a single unit with UUID and external ID
- 🎨 **Colored Console Output**: Errors in red, warnings in yellow, success in green, info in blue
- 🔌 **Pluggable Architecture**: Easy to extend with custom destinations
- 🚀 **Async Support**: Built with Tokio for high-performance async logging
- 📊 **Multiple Destinations**: Log to console, MongoDB, and PostgreSQL simultaneously

### Log Levels
- **Error** (Red) - Critical errors that need immediate attention
- **Warning** (Yellow) - Important notices that don't stop execution
- **Info** (Blue) - General information messages
- **Success** (Green) - Successful operation confirmations

### Supported Destinations
- **Console** (default) - Always available, colored output
- **MongoDB** - Document-based storage with flexible schema
- **PostgreSQL** - Relational database with structured tables

## Installation

Add IronScribe to your `Cargo.toml`: