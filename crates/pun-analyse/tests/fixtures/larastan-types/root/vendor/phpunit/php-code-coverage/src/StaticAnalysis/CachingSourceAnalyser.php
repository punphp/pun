<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\StaticAnalysis;

use const DIRECTORY_SEPARATOR;
use function file_get_contents;
use function file_put_contents;
use function hash;
use function implode;
use function is_file;
use function serialize;
use function unserialize;
use SebastianBergmann\CodeCoverage\Util\Filesystem;
use SebastianBergmann\CodeCoverage\Util\PhpParserVersion;
use SebastianBergmann\CodeCoverage\Version;

/**
 * @internal This interface is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class CachingSourceAnalyser implements SourceAnalyser
{
    /**
     * Version of the on-disk format of the serialized AnalysisResult objects.
     *
     * This must be incremented whenever the shape of AnalysisResult (or a value
     * object it aggregates) changes, so that cache entries written by a build
     * that reports the same Version::id() (for instance two snapshots of the
     * same development branch installed without git metadata) cannot be
     * misread after such a change.
     */
    private const int CACHE_FORMAT_VERSION = 3;

    /**
     * @var non-empty-string
     */
    private readonly string $directory;
    private readonly SourceAnalyser $sourceAnalyser;

    /**
     * @var non-negative-int
     */
    private int $cacheHits = 0;

    /**
     * @var non-negative-int
     */
    private int $cacheMisses = 0;

    /**
     * @param non-empty-string $directory
     */
    public function __construct(string $directory, SourceAnalyser $sourceAnalyser)
    {}

    /**
     * @param non-empty-string $sourceCodeFile
     */
    public function analyse(string $sourceCodeFile, string $sourceCode, bool $useAnnotationsForIgnoringCode, bool $ignoreDeprecatedCode): AnalysisResult
    {}

    /**
     * @return non-negative-int
     */
    public function cacheHits(): int
    {}

    /**
     * @return non-negative-int
     */
    public function cacheMisses(): int
    {}

    /**
     * @param non-empty-string $cacheFile
     */
    private function read(string $cacheFile): AnalysisResult|false
    {}

    /**
     * @param non-empty-string $cacheFile
     */
    private function write(string $cacheFile, AnalysisResult $result): void
    {}

    /**
     * @return non-empty-string
     */
    private function cacheFile(string $source, bool $useAnnotationsForIgnoringCode, bool $ignoreDeprecatedCode): string
    {}
}
