# Upload images server

[![Build Status](https://travis-ci.org/ChugunovRoman/uploadImages.svg?branch=master)](https://travis-ci.org/ChugunovRoman/uploadImages)

Rust server for upload images.

## Features:
* Uploads an image
* Uploads base64 of a supported image
* Downloads an image from URI
* Uploads JSON array with base64
* Uploads images via **multipart/form-data** (accept URI, binary or base64 data or all together)
* Creates thumbnail for each image

## Supported image format:
* jpeg
* png
* bmp
* ico
* tiff

## Run
You can run up the app via docker, just enter in terminal:
```bash
docker-compose up
```
