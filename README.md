# Love2D Ultralight

A lightweight WebView integration for [Love2D](https://love2d.org/) using [Ultralight](https://ultralig.ht/).

## Requirements
  - [Love2D](https://love2d.org/) version 11.3 or later.
  - [Ultralight SDK](https://ultralig.ht/) version 1.4.0.

## Documentation
  - [API Reference](https://amir-al-mohamad.github.io/love2d-ultralight/)

## Quick Start

1. **Download Ultralight SDK for your platform:**
  - [Windows x64](https://ultralight-sdk-dev.sfo2.cdn.digitaloceanspaces.com/ultralight-sdk-158d65c-win-x64.7z)
  - [Linux x64](https://ultralight-sdk-dev.sfo2.cdn.digitaloceanspaces.com/ultralight-sdk-158d65c-linux-x64.7z)
  - For macOS, see the [Universal Binary Build Instructions](#macos-universal-binary-universal-dylib-build-instructions).

2. **Extract the SDK** and copy all `.dll` files from the `bin` folder in the archive to the same directory as the module

3. **Usage example:**  
  See a usage example in the [documentation](https://amir-al-mohamad.github.io/love2d-ultralight/examples/usage.lua.html).

## macOS Universal Binary (Universal `.dylib`) Build Instructions

To support both Apple Silicon (ARM64) and Intel (x86_64) Macs, you can create a universal `.dylib` by combining the two architectures using `lipo`.

1. Download and extract the Ultralight SDK for **macOS ARM64** and **macOS x64**.
  - [macOS ARM64](https://ultralight-sdk-dev.sfo2.cdn.digitaloceanspaces.com/ultralight-sdk-158d65c-mac-arm64.7z)
  - [macOS x64](https://ultralight-sdk-dev.sfo2.cdn.digitaloceanspaces.com/ultralight-sdk-158d65c-mac-x64.7z)

2. Run these commands to create universal `.dylib` files:

  ```bash
    lipo -create path/to/mac-arm64/libAppCore.dylib path/to/mac-x64/libAppCore.dylib -output path/to/universal/libAppCore.dylib
    lipo -create path/to/mac-arm64/libUltralight.dylib path/to/mac-x64/libUltralight.dylib -output path/to/universal/libUltralight.dylib
    lipo -create path/to/mac-arm64/libUltralightCore.dylib path/to/mac-x64/libUltralightCore.dylib -output path/to/universal/libUltralightCore.dylib
    lipo -create path/to/mac-arm64/libWebCore.dylib path/to/mac-x64/libWebCore.dylib -output path/to/universal/libWebCore.dylib

    strip -x path/to/universal/*.dylib
  ```

3. Copy the resulting universal `.dylib` files to the same directory as the module.
