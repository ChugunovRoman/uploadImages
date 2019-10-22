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
After run, the server starts on localhost:8000.
The server have got one endpoint http://localhost:8000/api/v1/images/upload which accepts application/json, text/plain or multipart/form-data content type

All files store in /tmp directory. The app automatically create *uploaded_files* folder in the tmp directory and save all files into this folder. You can replace the tmp directory on own folder via STOREDIR environment variable. Just change the environment variable in the docker-compose.yml file

## Examples of requests

**Send one binary file:**
```bash
curl -X POST -H "Content-Type: image/jpeg" --data-binary "@./tests/dataset/jpg.jpg" http://localhost:8000/api/v1/images/upload
```

**Send one file as base64:**
```bash
base64 ./tests/dataset/png.png | curl -X POST -H "Content-Type: text/plain" -d @- http://localhost:8000/api/v1/images/upload
```

**Send one url:**
```bash
curl -X POST -H "Content-Type: text/plain" -d "https://cdn.pixabay.com/photo/2018/01/14/23/12/nature-3082832_960_720.jpg" http://localhost:8000/api/v1/images/upload
```

**Send JSON (Array\<string\>) with base64:**
```bash
cat ./tests/dataset/json.json | curl -X POST -H "Content-Type: application/json" -d @- http://localhost:8000/api/v1/images/upload
```

**Send multipart/form-data, multiple files (binary, base64, or url or all together):**
```bash
curl -X POST -H "Content-Type: multipart/form-data" -F 'files[]=https://cdn.pixabay.com/photo/2018/01/14/23/12/nature-3082832_960_720.jpg' -F 'files[]=@./tests/dataset/png.png' -F "files[]=`cat ./tests/dataset/jpg.base64`"  http://localhost:8000/api/v1/images/upload
```
## Author

*  [Chugunov Roman](https://github.com/ChugunovRoman)
