<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Html;

use const ENT_QUOTES;
use const ENT_XML1;
use function array_filter;
use function array_map;
use function ceil;
use function floor;
use function htmlspecialchars;
use function log10;
use function max;
use function sprintf;
use function sqrt;
use function usort;
use SebastianBergmann\CodeCoverage\Report\Thresholds;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class BubbleChart
{
    private Thresholds $thresholds;

    public function __construct(Thresholds $thresholds)
    {}

    /**
     * @param list<array{name: string, coverage: float|int, executableLines: int, complexity: int, link: string}> $items
     */
    public function render(array $items): string
    {}

    private function colorLevel(float $percent): string
    {}
}
