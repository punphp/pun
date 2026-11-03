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

use function array_shift;
use function array_unique;
use function sort;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class RangeEncoder
{
    /**
     * @param list<int> $lines
     *
     * @return list<non-empty-string>
     */
    public static function encode(array $lines): array
    {}

    /**
     * @return non-empty-string
     */
    public static function encodeBounds(int $start, int $end): string
    {}
}
