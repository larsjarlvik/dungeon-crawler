<img alt="banner" src="banner.jpg">

# Dungeon Crawler
Welcome to Dungeon Crawler (working name). A cross platform RPG game written in rust.

Tested platforms:
* Windows
* Linux
* Mac OS (Intel / M1)
* Android

## Development

### Compile
#### Desktop
```
cargo run
cargo run --release
```

#### Android
* Install Android SDK
* Set `$ANDROID_SDK_ROOT` environment variable

```
cargo apk run -p dungeon-crawler
cargo apk run --release -p dungeon-crawler
```

##### Print logs:
```
adb logcat RustStdoutStderr:D \*:S
```
