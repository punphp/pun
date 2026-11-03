<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\TestRunHistory;

use const DIRECTORY_SEPARATOR;
use const LOCK_EX;
use const LOCK_UN;
use function array_keys;
use function dirname;
use function fclose;
use function file_get_contents;
use function flock;
use function fopen;
use function ftruncate;
use function fwrite;
use function is_array;
use function is_dir;
use function is_file;
use function is_float;
use function is_int;
use function is_string;
use function json_decode;
use function json_encode;
use function rewind;
use function stream_get_contents;
use PHPUnit\Framework\TestStatus\TestStatus;
use PHPUnit\Runner\DirectoryDoesNotExistException;
use PHPUnit\Runner\Exception;
use PHPUnit\Util\Filesystem;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DefaultTestRunHistory implements TestRunHistory
{
    private const int VERSION = 2;

    /**
     * This filename is only used when no cache directory is configured. It is
     * intentionally not renamed along with the feature: it is commonly listed
     * in .gitignore files and renaming it would leave an untracked file behind.
     */
    private const string DEFAULT_FILENAME = '.phpunit.result.cache';
    private readonly string $filename;

    /**
     * @var array<string, TestStatus>
     */
    private array $defects = [];

    /**
     * @var array<string, float>
     */
    private array $times = [];

    /**
     * @var array<string, true>
     */
    private array $changedDefects = [];

    /**
     * @var array<string, true>
     */
    private array $changedTimes = [];

    public function __construct(string $filepath)
    {}

    public function setStatus(TestRunHistoryId $id, TestStatus $status): void
    {}

    public function remove(TestRunHistoryId $id): void
    {}

    public function status(TestRunHistoryId $id): TestStatus
    {}

    public function setTime(TestRunHistoryId $id, float $time): void
    {}

    public function time(TestRunHistoryId $id): float
    {}

    public function mergeWith(self $other): void
    {}

    public function load(): void
    {}

    /**
     * @throws Exception
     */
    public function persist(): void
    {}

    /**
     * Persists only the entries that were touched since load(). This drops
     * the entries of tests that no longer exist and must only be used when
     * the current test run executed every test that exists.
     *
     * @throws Exception
     */
    public function persistAndPrune(): void
    {}

    /**
     * @throws Exception
     */
    private function writeToFile(bool $prune): void
    {}

    /**
     * @return ?array{0: array<string, TestStatus>, 1: array<string, float>}
     */
    private function parse(string $contents): ?array
    {}
}
