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

use function array_fill;
use function count;
use function intdiv;

/**
 * Linear-space variant of the Myers diff algorithm (Eugene W. Myers, 1986,
 * "An O(ND) Difference Algorithm and Its Variations", §4b).
 *
 * Time:   O((N + M) * D)
 * Space:  O(N + M)
 *
 * Where N and M are the input lengths and D is the edit distance.
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class MyersDiff
{
    /**
     * @param list<mixed> $from
     * @param list<mixed> $to
     *
     * @return list<array{0: mixed, 1: int}> edit script as [token, Differ::OLD|ADDED|REMOVED] entries
     */
    public function calculate(array $from, array $to): array
    {}

    /**
     * Within each contiguous run of non-matching entries, emit all REMOVED
     * tokens before any ADDED tokens. The Myers algorithm may interleave them
     * depending on path tie-breaking; normalizing matches the conventional
     * diff(1) output ordering.
     *
     * @param list<array{0: mixed, 1: int}> $diff
     *
     * @return list<array{0: mixed, 1: int}>
     */
    private function normalize(array $diff): array
    {}

    /**
     * @param list<mixed>                   $a
     * @param list<mixed>                   $b
     * @param list<array{0: mixed, 1: int}> $out
     */
    private function diffRange(array $a, int $aLo, int $aHi, array $b, int $bLo, int $bHi, array &$out): void
    {}

    /**
     * Find the middle snake of an optimal D-path through the edit graph using
     * bidirectional search.
     *
     * @param list<mixed> $a
     * @param list<mixed> $b
     *
     * @return array{0: int, 1: int, 2: int, 3: int} [xStart, yStart, xEnd, yEnd] in coords relative to (aLo, bLo)
     */
    private function findMiddleSnake(array $a, int $aLo, int $aHi, array $b, int $bLo, int $bHi): array
    {}
}
