# Upload images server

[![Build Status](https://travis-ci.org/ChugunovRoman/uploadImages.svg?branch=master)](https://travis-ci.org/ChugunovRoman/uploadImages)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/30055259fcc84fd0a2e7c7adf5f69c39)](https://www.codacy.com/manual/ChugunovRoman/uploadImages?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=ChugunovRoman/uploadImages&amp;utm_campaign=Badge_Grade)

Rust server for upload images.

## Features
*   Uploads an image
*   Uploads base64 of a supported image
*   Downloads an image from URI
*   Uploads JSON array with base64
*   Uploads images via **multipart/form-data** (accept URI, binary or base64 data or all together)
*   Creates thumbnail for each image

## Supported image format
*   jpeg
*   png
*   bmp
*   ico
*   tiff

## Run
You can run up the app via docker, just enter in terminal:
```bash
docker-compose up
```
