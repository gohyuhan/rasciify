# rasciify

rasciify was design to transform images into ASCII art. It was developed in Rust, where you can provide image to generate RGB, grayscale, or text-based artwork within any rust program.

![badge](https://img.shields.io/badge/License-MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rust-1.83.0-blue)
![Rasciify](https://img.shields.io/github/v/release/gohyuhan/rasciify)


## Table of Contents
1. [Description](#description)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Change Logs](#change-logs)

## Description
This project ``rasciify`` allows you to easily generate ASCII art within a rust program by providing an image. It provides a simple and easy-to-use interface for performing various ASCII art generation, by providing an image, you can choose to obtain the generated ASCII art in the form of RGB image, grayscale image or text.  

If you need further modification, you can also choose to obtain it in the form of ``String`` or ``ImageBuffer``. Check [here](docs/usage.md) to learn more.

## Installation
To install gnupg, you can use the following command:
To add ``rasciify`` to be use in a rust program
```bash
cargo add rasciify
```

## Usage
To check out how to use ``rasciify``, check the [docs](docs/usage.md) here

## Change Logs
### v0.1.0
Released: 2025-02-14 
Initial crate Publish. This includes operation like:

- image to text based ASCII art
- image to RGB image ASCII art
- image to grayscale image ASCII art