# imgsort
**Automatically sort and deduplicate your photographs based on their EXIF metadata.**

This tool makes automated image organization workflows simple and straightforward,
allowing you to manage large image libraries with minimal effort.

## Features
- **Automatic Organization**: Automatically sort your images into folders based on their EXIF metadata.
- **Deduplication**: Automatically detect and remove duplicate images from your library based on the exact binary content of the image file.
- **Customizable**: Configure how your images are sorted using a simple configuration file.

## Usage
```bash
# Run in audit mode to see what the tool will do without making changes to your library
imgsort -c config.yaml --audit

# Run in normal mode to actually sort your images
imgsort -c config.yaml
```

## Configuration
```toml
source = "/Volumes/photo/Import"
target = "/Volumes/photo"
template = "{year}/{date}T{time}-{camera.model}"
synology = true # Set to true if you are running on a Synology NAS
```

## Template Variables
- `{name}`: The name of the image file.
- `{year}`: The year the image was taken.
- `{month}`: The month the image was taken.
- `{day}`: The day the image was taken.
- `{date}`: The date the image was taken in the format `YYYY-MM-DD`.
- `{time}`: The time the image was taken in the format `HHMMSS`.
- `{camera.make}`: The manufacturer of the camera which took the image.
- `{camera.model}`: The model of the camera which took the image.
- `{lens.make}`: The manufacturer of the lens which took the image.
- `{lens.model}`: The model of the lens which took the image.
- `{artist}`: The artist who created the image.
- `{copyright}`: The copyright holder of the image.
