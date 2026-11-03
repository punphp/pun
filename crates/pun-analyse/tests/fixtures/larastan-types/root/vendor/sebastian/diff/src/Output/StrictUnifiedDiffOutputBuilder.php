<?php declare(strict_types=1);
/*
 * This file is part of sebastian/diff.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Diff\Output;

use function array_merge;
use function array_splice;
use function assert;
use function count;
use function fclose;
use function fopen;
use function fwrite;
use function is_bool;
use function is_int;
use function is_resource;
use function is_string;
use function max;
use function min;
use function sprintf;
use function str_ends_with;
use function stream_get_contents;
use function substr;
use SebastianBergmann\Diff\ConfigurationException;
use SebastianBergmann\Diff\Differ;

/**
 * Strict Unified diff output builder.
 *
 * Generates (strict) Unified diff's (unidiffs) with hunks.
 */
final class StrictUnifiedDiffOutputBuilder implements DiffOutputBuilderInterface
{
    /**
     * @var array{addLineNumbers: bool, collapseRanges: bool, commonLineThreshold: int, contextLines: int, emitDiffLineEndWarning: bool, emitNoLineEndEofWarning: bool, fromFile: null|string, fromFileDate: null|string, header: null|string, toFile: null|string, toFileDate: null|string}
     */
    private static array $default = [
        'addLineNumbers'          => true,  // when false, hunk header is rendered as `@@ @@` (no line numbers); when true, as `@@ -from,range +to,range @@`
        'collapseRanges'          => true,  // ranges of length one are rendered with the trailing `,1`
        'commonLineThreshold'     => 6,     // number of same lines before ending a new hunk and creating a new one (if needed)
        'contextLines'            => 3,     // like `diff:  -u, -U NUM, --unified[=NUM]`, for patch/git apply compatibility best to keep at least @ 3
        'emitDiffLineEndWarning'  => false, // when true, the `#Warning: Strings contain different line endings!` line inserted by `Differ` is rendered (it is not part of strict unified diff format)
        'emitNoLineEndEofWarning' => true,  // when false, the `\ No newline at end of file` marker is suppressed (useful for comparisons not related to files)
        'fromFile'                => null,
        'fromFileDate'            => null,
        'header'                  => null,  // when provided, used verbatim as the header; otherwise built from `fromFile`/`fromFileDate`/`toFile`/`toFileDate`
        'toFile'                  => null,
        'toFileDate'              => null,
    ];
    private bool $addLineNumbers;
    private bool $changed;
    private bool $collapseRanges;

    /**
     * @var positive-int
     */
    private int $commonLineThreshold;
    private bool $emitDiffLineEndWarning;
    private bool $emitNoLineEndEofWarning;
    private string $header;

    /**
     * @var int<0, max>
     */
    private int $contextLines;

    /**
     * @param array<string, mixed> $options
     */
    public function __construct(array $options = [])
    {}

    /**
     * @param list<array{0: mixed, 1: int}> $diff
     */
    public function getDiff(array $diff): string
    {}

    /**
     * @param non-empty-list<array{0: mixed, 1: int}> $diff
     */
    private function writeDiffHunks(mixed $output, array $diff): void
    {}

    /**
     * @param list<array{0: mixed, 1: int}> $diff
     */
    private function writeHunk(
        array $diff,
        int $diffStartIndex,
        int $diffEndIndex,
        int $fromStart,
        int $fromRange,
        int $toStart,
        int $toRange,
        mixed $output
    ): void {}

    /**
     * @param array<string, mixed> $options
     */
    private function assertString(array $options, string $option): void
    {}

    /**
     * @param array<string, mixed> $options
     */
    private function assertStringOrNull(array $options, string $option): void
    {}
}
