<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\TestIndex;

use const DIRECTORY_SEPARATOR;
use const LOCK_EX;
use const PHP_VERSION_ID;
use function count;
use function dirname;
use function file_get_contents;
use function file_put_contents;
use function is_array;
use function is_bool;
use function is_dir;
use function is_file;
use function is_int;
use function is_string;
use function json_decode;
use function json_encode;
use function realpath;
use PHPUnit\Framework\TestCase;
use PHPUnit\Runner\DirectoryDoesNotExistException;
use PHPUnit\Runner\Exception;
use PHPUnit\Runner\Version;
use PHPUnit\Util\Filesystem;
use ReflectionClass;

/**
 * What is known about the tests in a set of test files without loading them.
 *
 * The index makes it possible to decide that a test file cannot contribute a
 * test to a run before the file is loaded. An entry is only handed out while
 * every source file it was derived from is unchanged, and the index as a whole
 * is discarded when it was written by a different version of PHPUnit, or by a
 * different version of PHP, as the meaning of the metadata it stores is defined
 * by both: a test class can be declared conditionally, and the metadata for it
 * can depend on the version of PHP that reads the file, while the file itself is
 * unchanged.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TestIndex
{
    private const int VERSION                   = 4;
    private const string DEFAULT_INDEX_FILENAME = 'test-index';
    private readonly string $indexFilename;
    private readonly FileHasher $hasher;

    /**
     * @var array<non-empty-string, TestIndexEntry>
     */
    private array $entries = [];

    public function __construct(string $filepath, ?FileHasher $hasher = null)
    {}

    /**
     * Returns null when the file is not indexed or when the entry for it is no
     * longer valid. Callers must load the file in that case.
     */
    public function entryFor(string $file): ?TestIndexEntry
    {}

    /**
     * @param ReflectionClass<TestCase> $class
     */
    public function record(ReflectionClass $class, bool $madePhpUnitWarn): void
    {}

    public function load(): void
    {}

    /**
     * Entries that were not recorded during this run are written back
     * unchanged: a run that skipped a file did not learn anything about it and
     * must not cause what is known about it to be forgotten. Entries for files
     * that no longer exist are dropped.
     *
     * @throws Exception
     */
    public function persist(): void
    {}

    /**
     * @param array<int, non-empty-string> $groupNames
     */
    private static function entryFromArray(array $groupNames, mixed $entry): ?TestIndexEntry
    {}
}
