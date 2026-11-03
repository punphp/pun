<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report;

use const PHP_EOL;
use function array_map;
use function date;
use function ksort;
use function max;
use function sprintf;
use function str_pad;
use function strlen;
use SebastianBergmann\CodeCoverage\Node\Directory;
use SebastianBergmann\CodeCoverage\Node\File;
use SebastianBergmann\CodeCoverage\Util\Percentage;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Text
{
    private const string COLOR_GREEN  = "\x1b[30;42m";
    private const string COLOR_YELLOW = "\x1b[30;43m";
    private const string COLOR_RED    = "\x1b[37;41m";
    private const string COLOR_HEADER = "\x1b[1;37;40m";
    private const string COLOR_RESET  = "\x1b[0m";
    private Thresholds $thresholds;
    private bool $showUncoveredFiles;
    private bool $showOnlySummary;

    public function __construct(Thresholds $thresholds, bool $showUncoveredFiles = false, bool $showOnlySummary = false)
    {}

    public function process(Directory $report, bool $showColors = false): string
    {}

    private function coverageColor(int $numberOfCoveredElements, int $totalNumberOfElements): string
    {}

    private function printCoverageCounts(int $numberOfCoveredElements, int $totalNumberOfElements, int $precision): string
    {}

    private function format(string $color, int $padding, false|string $string): string
    {}
}
