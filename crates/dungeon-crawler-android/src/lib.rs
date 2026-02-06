// This crate exists solely to build the Android APK.
// The android_main entry point is defined in dungeon-crawler with #[no_mangle],
// so we just need to ensure dungeon-crawler is linked.
use dungeon_crawler as _;
