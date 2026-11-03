<?php declare(strict_types = 1);

namespace PHPStan;

use Composer\Autoload\ClassLoader;
use function class_exists;
use const PHP_VERSION_ID;

final class PharAutoloader
{
	/** @var ClassLoader */
	private static $composerAutoloader;

	/** @var bool */
	private static $polyfillsLoaded = false;

	final public static function loadClass(string $class): void {}
}

spl_autoload_register([PharAutoloader::class, 'loadClass']);
