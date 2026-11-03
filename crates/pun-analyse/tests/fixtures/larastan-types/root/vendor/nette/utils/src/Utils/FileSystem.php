<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function sprintf, strlen;
use const DIRECTORY_SEPARATOR;


/**
 * File system tool.
 */
final class FileSystem
{
	/**
	 * Creates a directory if it does not exist, including parent directories.
	 * @throws Nette\IOException  on error occurred
	 */
	public static function createDir(string $dir, int $mode = 0o777): void
	{}


	/**
	 * Copies a file or an entire directory. Overwrites existing files and directories by default.
	 * @throws Nette\IOException  on error occurred
	 * @throws Nette\InvalidStateException  if $overwrite is set to false and destination already exists
	 */
	public static function copy(string $origin, string $target, bool $overwrite = true): void
	{}


	/**
	 * Opens file and returns resource.
	 * @return resource
	 * @throws Nette\IOException  on error occurred
	 */
	public static function open(string $path, string $mode)
	{}


	/**
	 * Deletes a file or an entire directory if exists. If the directory is not empty, it deletes its contents first.
	 * @throws Nette\IOException  on error occurred
	 */
	public static function delete(string $path): void
	{}


	/**
	 * Renames or moves a file or a directory. Overwrites existing files and directories by default.
	 * @throws Nette\IOException  on error occurred
	 * @throws Nette\InvalidStateException  if $overwrite is set to false and destination already exists
	 */
	public static function rename(string $origin, string $target, bool $overwrite = true): void
	{}


	/**
	 * Reads the content of a file.
	 * @throws Nette\IOException  on error occurred
	 */
	public static function read(string $file): string
	{}


	/**
	 * Reads the file content line by line. Because it reads continuously as we iterate over the lines,
	 * it is possible to read files larger than the available memory.
	 * @return \Generator<int, string>
	 * @throws Nette\IOException  on error occurred
	 */
	public static function readLines(string $file, bool $stripNewLines = true): \Generator
	{}


	/**
	 * Writes the string to a file. Creates the parent directory if it does not exist. Pass null as $mode to skip chmod.
	 * @throws Nette\IOException  on error occurred
	 */
	public static function write(string $file, string $content, ?int $mode = 0o666): void
	{}


	/**
	 * Writes the string to a file atomically: the content is written to a temporary file, which then replaces
	 * the target, so a concurrent reader never sees the file partially written or truncated.
	 * Creates the parent directory if it does not exist. Pass null as $mode to skip chmod.
	 * @throws Nette\IOException  on error occurred
	 */
	public static function writeAtomic(string $file, string $content, ?int $mode = 0o666): void
	{}


	/**
	 * Sets file permissions to `$fileMode` or directory permissions to `$dirMode`.
	 * Recursively traverses and sets permissions on the entire contents of the directory as well.
	 * @throws Nette\IOException  on error occurred
	 */
	public static function makeWritable(string $path, int $dirMode = 0o777, int $fileMode = 0o666): void
	{}


	/**
	 * Determines if the path is absolute.
	 */
	public static function isAbsolute(string $path): bool
	{}


	/**
	 * Determines whether the string is a valid cross-platform filename without any path information.
	 */
	public static function isValidFilename(string $name): bool
	{}


	/**
	 * Normalizes `..` and `.` and directory separators in path; a scheme like `phar://` is kept.
	 */
	public static function normalizePath(string $path): string
	{}


	/**
	 * Joins all segments of the path and normalizes the result.
	 */
	public static function joinPaths(string ...$paths): string
	{}


	/**
	 * Resolves a path against a base path. If the path is absolute, returns it directly, if it's relative, joins it with the base path.
	 */
	public static function resolvePath(string $basePath, string $path): string
	{}


	/**
	 * Converts backslashes to slashes.
	 */
	public static function unixSlashes(string $path): string
	{}


	/**
	 * Converts slashes to platform-specific directory separators.
	 */
	public static function platformSlashes(string $path): string
	{}
}
