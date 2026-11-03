<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output;

use const PHP_EOL;
use function sprintf;
use PHPUnit\TestRunner\TestResult\TestResult;
use PHPUnit\Util\Color;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class SummaryPrinter
{
    private readonly Printer $printer;
    private readonly bool $colors;
    private bool $countPrinted = false;

    public function __construct(Printer $printer, bool $colors)
    {}

    public function print(TestResult $result): void
    {}

    /**
     * @param non-empty-string $name
     * @param non-empty-string $color
     */
    private function printCountString(int $count, string $name, string $color, bool $always = false): void
    {}

    /**
     * @param non-empty-string $color
     * @param non-empty-string $buffer
     */
    private function printWithColor(string $color, string $buffer, bool $lf = true): void
    {}

    private function printNumberOfIssuesIgnoredByBaseline(TestResult $result): void
    {}
}
