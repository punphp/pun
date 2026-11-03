<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Jsonl;

use const DIRECTORY_SEPARATOR;
use const JSON_PRETTY_PRINT;
use const JSON_UNESCAPED_SLASHES;
use const JSON_UNESCAPED_UNICODE;
use const SORT_STRING;
use function array_any;
use function count;
use function fclose;
use function fopen;
use function fwrite;
use function is_dir;
use function is_file;
use function is_writable;
use function ksort;
use function str_replace;
use function str_starts_with;
use function strcmp;
use function strlen;
use function substr;
use function usort;
use DateTimeImmutable;
use DateTimeInterface;
use Generator;
use SebastianBergmann\CodeCoverage\Data\ProcessedBranchCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionCoverageData;
use SebastianBergmann\CodeCoverage\JsonException;
use SebastianBergmann\CodeCoverage\Node\Directory as DirectoryNode;
use SebastianBergmann\CodeCoverage\Node\File as FileNode;
use SebastianBergmann\CodeCoverage\PathExistsButIsNotDirectoryException;
use SebastianBergmann\CodeCoverage\Util\EnsuresUtf8;
use SebastianBergmann\CodeCoverage\Util\Filesystem;
use SebastianBergmann\CodeCoverage\Util\Json;
use SebastianBergmann\CodeCoverage\Version;
use SebastianBergmann\CodeCoverage\WriteOperationFailedException;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @phpstan-import-type TestIndexType from ProcessedCodeCoverageData
 */
final class Facade
{
    use EnsuresUtf8;

    /**
     * Incremented only when a change to the format breaks a consumer that
     * relies on this version. Adding an optional key does not break one.
     */
    private const int SCHEMA_VERSION = 1;

    /**
     * @param non-empty-string $target
     *
     * @throws JsonException
     * @throws PathExistsButIsNotDirectoryException
     * @throws WriteOperationFailedException
     */
    public function process(DirectoryNode $report, string $target, ?DateTimeImmutable $generatedAt = null): void
    {}

    /**
     * @param array<string, FileNode> $files
     */
    private function collectedBranchCoverage(array $files): bool
    {}

    /**
     * @return array<string, FileNode>
     */
    private function files(DirectoryNode $report, string $sourceRoot): array
    {}

    /**
     * @param non-empty-string               $target
     * @param iterable<array<string, mixed>> $records
     *
     * @throws JsonException
     * @throws WriteOperationFailedException
     */
    private function writeRecords(string $target, iterable $records): void
    {}

    /**
     * @param array<string, FileNode> $files
     *
     * @return Generator<int, array<string, mixed>>
     */
    private function coverageRecords(array $files): Generator
    { yield; }

    /**
     * @param array<string, FileNode> $files
     *
     * @return Generator<int, array<string, mixed>>
     */
    private function testRecords(array $files): Generator
    { yield; }

    /**
     * @param array<string, FileNode> $files
     *
     * @return array<non-empty-string, array<string, list<non-empty-string>>>
     */
    private function coveredLinesByTest(array $files): array
    {}

    /**
     * @param non-empty-string        $target
     * @param array<string, FileNode> $files
     *
     * @throws JsonException
     * @throws WriteOperationFailedException
     */
    private function writeMeta(string $target, DirectoryNode $report, string $sourceRoot, array $files, DateTimeImmutable $generatedAt): void
    {}

    /**
     * @return array<string, mixed>
     */
    private function fileRecord(FileNode $file, string $path): array
    {}

    /**
     * @param array<positive-int, ?array<TestIndexType, positive-int>> $lineCoverage
     *
     * @return list<positive-int>
     */
    private function uncoveredLines(array $lineCoverage): array
    {}

    /**
     * @param array<positive-int, ?array<TestIndexType, positive-int>> $lineCoverage
     *
     * @return list<array<string, mixed>>
     */
    private function symbols(FileNode $file, array $lineCoverage): array
    {}

    /**
     * @param array<positive-int, ?array<TestIndexType, positive-int>> $lineCoverage
     * @param ?array<int, ProcessedBranchCoverageData>                 $branches     null when the driver collected no branch coverage
     *
     * @return ?array<string, mixed>
     */
    private function symbolRecord(string $name, int $startLine, int $endLine, array $lineCoverage, ?array $branches): ?array
    {}

    /**
     * @param array<non-empty-string, ProcessedFunctionCoverageData> $functionCoverage
     *
     * @return ?array<int, ProcessedBranchCoverageData>
     */
    private function branchesOf(FileNode $file, array $functionCoverage, string $key): ?array
    {}

    private function relativePath(string $path, string $sourceRoot): string
    {}

    /**
     * @param array<string, mixed> $record
     *
     * @throws JsonException
     */
    private function encode(array $record): string
    {}

    /**
     * @throws PathExistsButIsNotDirectoryException
     * @throws WriteOperationFailedException
     */
    private function initTargetDirectory(string $directory): void
    {}
}
