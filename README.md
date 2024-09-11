# Rename image file with rexiv2

## Description
Rename image file names into `<timestamp>_<date>_<time>.CR3` format in batch using exif data.

## Background

For backup purposes. I will rename my image raw files with the date/time it took before upload. 

For example, a image a camera took (e.g. IMG_1234.CR3 for canon) might overlap with previous image with same name. So I would rename the raw file with datetime info for de-duplicate purposes. So the IMG_1234 will become `<timestamp>_<date>_<time>.CR3` (e.g. 1723611712900-20240814-1301.CR3 is one that was took 2024 Aug 14, 13:01 in the afternoon). This way, I can arrange my images by time with ease. 

I initially use the [`exiftool` by phil](https://exiftool.org/), but the performance is a bit slow. So I then seek solution for a rust-version exif tool for renaming these raw files. 


## Install from source

- build the binary with `$ cargo build --release` 
- for system-wise usage:  `$ mv $(pwd)/target/exifrename /usr/local/bin`

## Test 

- put any sample jpg/JPG/CR3 file in current folder.
- run with `cargo run`

## Usage

`$ exifrename` to run on current folder
`$ exifrename --dry-run` to check run 
`$ exifrename <dir>` to run for certain folder.

## Limitation

- Timezone is set to +0800
- Supported file format is currently jpg/JPG/CR3

## Considerations

- I don't know anything about Rust
- Use `rexiv2` for exif parsing with rust
- Use `chrono` for timezone info for timestamp. current timezone is set at east+8
- Use `tokio` for better memory efficiency (not perfed). (All tokio usage are provided by AI)

## TODO

- [ ] Pass timezone / file format as env variable / args
- [ ] Perf for tokio
