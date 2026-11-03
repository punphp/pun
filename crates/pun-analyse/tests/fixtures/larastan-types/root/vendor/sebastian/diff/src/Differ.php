<?php declare(strict_types=1);
/*
 * This file is part of sebastian/diff.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Diff;

use const PREG_SPLIT_DELIM_CAPTURE;
use const PREG_SPLIT_NO_EMPTY;
use function array_any;
use function array_unshift;
use function array_values;
use function count;
use function current;
use function end;
use function is_string;
use function key;
use function preg_split;
use function prev;
use function reset;
use function str_ends_with;
use function substr;
use SebastianBergmann\Diff\Output\DiffOutputBuilderInterface;

final class Differ
{
    public const int OLD                     = 0;
    public const int ADDED                   = 1;
    public const int REMOVED                 = 2;
    public const int DIFF_LINE_END_WARNING   = 3;
    public const int NO_LINE_END_EOF_WARNING = 4;
    private DiffOutputBuilderInterface $outputBuilder;

    public function __construct(DiffOutputBuilderInterface $outputBuilder)
    {}

    /**
     * @param array<int|string, int|string>|string $from
     * @param array<int|string, int|string>|string $to
     */
    public function diff(array|string $from, array|string $to): string
    {}

    /**
     * @param array<int|string, int|string>|string $from
     * @param array<int|string, int|string>|string $to
     *
     * @return list<array{0: mixed, 1: int}>
     */
    public function diffToArray(array|string $from, array|string $to): array
    {}

    /**
     * @return list<string>
     */
    private function splitStringByLines(string $input): array
    {}

    /**
     * @param list<array{0: mixed, 1: int}> $diff
     */
    private function detectUnmatchedLineEndings(array $diff): bool
    {}

    private function getLinebreak(int|string $line): string
    {}

    /**
     * @param array<int|string, int|string> $from
     * @param array<int|string, int|string> $to
     *
     * @return array{0: array<int|string, int|string>, 1: array<int|string, int|string>, 2: array<int|string, int|string>, 3: array<int|string, int|string>}
     */
    private static function getArrayDiffParted(array &$from, array &$to): array
    {}
}
