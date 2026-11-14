//! Pun's package manager: pun.lock, the download cache, and `vendor/` laid out as Composer lays it out, so the PHP a
//! project runs (Composer's autoloader, `Composer\InstalledVersions`, Laravel's `package:discover`) works unchanged.

pub mod autoload;
pub mod bins;
pub mod classmap;
pub mod fetch;
pub mod install;
pub mod installed;
pub mod lock;
pub mod paths;
pub mod php;
pub mod plugins;
pub mod scripts;
pub mod version;
