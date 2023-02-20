# Issue demonstration

Build and run the project, then use 7-Zip to attempt to decompress the file (`compressed.7z`). It will fail to decompress due to a data error. If using the `sevenz-rust` crate to decompress the file, a `dist overflow` error is returned, hence the name of this repository.

```shell
$ cargo run
   Compiling sevenz-overflow v0.1.0 (/Users/nfiedler/projects/sevenz-overflow)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/sevenz-overflow`
wrote SevenZArchiveEntry { name: "corrupted.pdf", has_stream: true, is_directory: false, is_anti_item: false, has_creation_date: false, has_last_modified_date: false, has_access_date: false, creation_date: FileTime(0), last_modified_date: FileTime(0), access_date: FileTime(0), has_windows_attributes: false, windows_attributes: 0, has_crc: true, crc: 1503102460, compressed_crc: 3693450089, size: 1272254, compressed_size: 1196883, content_methods: [SevenZMethodConfiguration { method: SevenZMethod("LZMA2", [33]), options: None }] }

$ 7zz e compressed.7z

7-Zip (z) 22.01 (arm64) : Copyright (c) 1999-2022 Igor Pavlov : 2022-07-15
 64-bit arm_v:8 locale=en_US.UTF-8 Threads:8, ASM

Scanning the drive for archives:
1 file, 1196989 bytes (1169 KiB)

Extracting archive: compressed.7z
--
Path = compressed.7z
Type = 7z
Physical Size = 1196989
Headers Size = 106
Method = LZMA2:23
Solid = -
Blocks = 1

ERROR: Data Error : corrupted.pdf

Sub items Errors: 1

Archives with Errors: 1

Sub items Errors: 1
```

Same issue occurs with other binary files, such as JPEG images.

If you notice anything that I am doing wrong, please let me know. Thank you.
