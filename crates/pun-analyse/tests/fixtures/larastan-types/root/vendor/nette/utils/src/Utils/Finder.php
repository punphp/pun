<?php declare(strict_types=1);

/**
 * This file is part of the Nette Framework (https://nette.org)
 * Copyright (c) 2004 David Grudl (https://davidgrudl.com)
 */

namespace Nette\Utils;

use Nette;
use function count, func_get_args, func_num_args, is_array, sprintf;
use const DIRECTORY_SEPARATOR, GLOB_NOESCAPE, GLOB_NOSORT, GLOB_ONLYDIR;


/**
 * Searches for files and directories in directory trees.
 *
 * Finder::findFiles('*.php')
 *     ->size('> 10kB')
 *     ->from('.')
 *     ->exclude('temp');
 *
 * @implements \IteratorAggregate<string, FileInfo>
 */
class Finder implements \IteratorAggregate
{
	/** @var array<array{string, string}> */
	private array $find = [];

	/** @var string[] */
	private array $in = [];

	/** @var array<\Closure(FileInfo): bool> */
	private array $filters = [];

	/** @var array<\Closure(FileInfo): bool> */
	private array $descentFilters = [];

	/** @var array<string|self> */
	private array $appends = [];
	private bool $childFirst = false;

	/** @var ?(\Closure(FileInfo, FileInfo): int) */
	private ?\Closure $sort = null;
	private int $maxDepth = -1;
	private bool $ignoreUnreadableDirs = true;


	/**
	 * Begins search for files and directories matching mask. The ** wildcard searches recursively;
	 * a trailing slash limits the mask to directories.
	 * @param  string|list<string>  $masks
	 */
	public static function find(string|array $masks = ['*']): static
	{}


	/**
	 * Begins search for files matching mask. The ** wildcard searches recursively.
	 * @param  string|list<string>  $masks
	 */
	public static function findFiles(string|array $masks = ['*']): static
	{}


	/**
	 * Begins search for directories matching mask. The ** wildcard searches recursively.
	 * @param  string|list<string>  $masks
	 */
	public static function findDirectories(string|array $masks = ['*']): static
	{}


	/**
	 * Finds files matching the specified masks.
	 * @param  string|list<string>  $masks
	 */
	public function files(string|array $masks = ['*']): static
	{}


	/**
	 * Finds directories matching the specified masks.
	 * @param  string|list<string>  $masks
	 */
	public function directories(string|array $masks = ['*']): static
	{}


	/** @param  list<string>  $masks */
	private function addMask(array $masks, string $mode): static
	{}


	private static function hasTrailingSeparator(string $mask): bool
	{}


	// Expands a ** that is not followed by a slash into **/*, so that e.g. "test/**" and "**.c" search recursively.
	private static function expandGlobStar(string $mask): string
	{}


	/**
	 * Searches in the given directories. Wildcards * and ? are allowed; unlike in masks, [ and ] are taken literally.
	 * @param  string|list<string>  $paths
	 */
	public function in(string|array $paths): static
	{}


	/**
	 * Searches recursively from the given directories. Wildcards * and ? are allowed; unlike in masks, [ and ] are taken literally.
	 * @param  string|list<string>  $paths
	 */
	public function from(string|array $paths): static
	{}


	/** @param  list<string>  $paths */
	private function addLocation(array $paths, string $ext): void
	{}


	/**
	 * Lists directory's contents before the directory itself. By default, this is disabled.
	 */
	public function childFirst(bool $state = true): static
	{}


	/**
	 * Ignores unreadable directories. By default, this is enabled.
	 */
	public function ignoreUnreadableDirs(bool $state = true): static
	{}


	/**
	 * Sets a comparison function for sorting entries within each directory.
	 * @param  callable(FileInfo, FileInfo): int  $callback
	 */
	public function sortBy(callable $callback): static
	{}


	/**
	 * Sorts files in each directory naturally by name.
	 */
	public function sortByName(): static
	{}


	/**
	 * Appends the specified file paths to results. Passing null creates and returns a new sub-finder whose results are appended.
	 * @param  string|list<string>|null  $paths
	 */
	public function append(string|array|null $paths = null): static
	{}


	/********************* filtering ****************d*g**/


	/**
	 * Skips entries that match the given masks, using the same grammar as find() masks, relative to the directories from in() or from().
	 * A trailing slash excludes directories only; a trailing /* or /** excludes the contents while keeping the directory itself.
	 * @param  string|list<string>  $masks
	 */
	public function exclude(string|array $masks): static
	{}


	/**
	 * Yields only entries which satisfy the given filter.
	 * @param  callable(FileInfo): bool  $callback
	 */
	public function filter(callable $callback): static
	{}


	/**
	 * It descends only to directories that match the specified filter.
	 * @param  callable(FileInfo): bool  $callback
	 */
	public function descentFilter(callable $callback): static
	{}


	/**
	 * Sets the maximum depth of entries.
	 */
	public function limitDepth(?int $depth): static
	{}


	/**
	 * Restricts the search by size. $operator accepts "[operator] [size] [unit]" example: >=10kB
	 * @param  '>'|'>='|'<'|'<='|'='|'=='|'==='|'!='|'!=='|'<>'  $operator  or predicate string
	 */
	public function size(string $operator, ?int $size = null): static
	{}


	/**
	 * Restricts the search by modified time. $operator accepts "[operator] [date]" example: >1978-01-23
	 * @param  '>'|'>='|'<'|'<='|'='|'=='|'==='|'!='|'!=='|'<>'  $operator  or predicate string
	 */
	public function date(string $operator, string|int|\DateTimeInterface|null $date = null): static
	{}


	/********************* iterator generator ****************d*g**/


	/**
	 * Returns an array with all found files and directories.
	 * @return list<FileInfo>
	 */
	public function collect(): array
	{}


	/** @return \Generator<string, FileInfo> */
	public function getIterator(): \Generator
	{ yield; }


	/**
	 * @param  array<object{pattern: string, mode: string, recursive: bool}>  $searches
	 * @param  string[]  $subdirs
	 * @return \Generator<string, FileInfo>
	 */
	private function traverseDir(string $dir, array $searches, array $subdirs = []): \Generator
	{ yield; }


	/** @param  iterable<string>  $pathNames */
	private function convertToFiles(iterable $pathNames, string $relativePath, bool $absolute): \Generator
	{ yield; }


	/**
	 * @param  (\Closure(FileInfo): bool)[]  $filters
	 * @param  array<int, bool>  $cache
	 */
	private function proveFilters(array $filters, FileInfo $file, array &$cache): bool
	{}


	/** @return array<string, array<object{pattern: string, mode: string, recursive: bool}>> */
	private function buildPlan(): array
	{}


	/**
	 * Since glob() does not know ** wildcard, we divide the path into a part for glob and a part for manual traversal.
	 * @return array{string, string, bool}
	 */
	private static function splitRecursivePart(string $path): array
	{}


	/**
	 * Converts wildcards to regular expression.
	 */
	private function buildPattern(string $mask): string
	{}
}
