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

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Colors
{
    private string $successLow;
    private string $successLowDark;
    private string $successMedium;
    private string $successMediumDark;
    private string $successHigh;
    private string $successHighDark;
    private string $successBar;
    private string $successBarDark;
    private string $warning;
    private string $warningDark;
    private string $warningBar;
    private string $warningBarDark;
    private string $danger;
    private string $dangerDark;
    private string $dangerBar;
    private string $dangerBarDark;
    private string $breadcrumbs;
    private string $breadcrumbsDark;

    public static function default(): self
    {}

    public static function from(string $successLow, string $successLowDark, string $successMedium, string $successMediumDark, string $successHigh, string $successHighDark, string $successBar, string $successBarDark, string $warning, string $warningDark, string $warningBar, string $warningBarDark, string $danger, string $dangerDark, string $dangerBar, string $dangerBarDark, string $breadcrumbs, string $breadcrumbsDark): self
    {}

    private function __construct(string $successLow, string $successLowDark, string $successMedium, string $successMediumDark, string $successHigh, string $successHighDark, string $successBar, string $successBarDark, string $warning, string $warningDark, string $warningBar, string $warningBarDark, string $danger, string $dangerDark, string $dangerBar, string $dangerBarDark, string $breadcrumbs, string $breadcrumbsDark)
    {}

    public function successLow(): string
    {}

    public function successLowDark(): string
    {}

    public function successMedium(): string
    {}

    public function successMediumDark(): string
    {}

    public function successHigh(): string
    {}

    public function successHighDark(): string
    {}

    public function successBar(): string
    {}

    public function successBarDark(): string
    {}

    public function warning(): string
    {}

    public function warningDark(): string
    {}

    public function warningBar(): string
    {}

    public function warningBarDark(): string
    {}

    public function danger(): string
    {}

    public function dangerDark(): string
    {}

    public function dangerBar(): string
    {}

    public function dangerBarDark(): string
    {}

    public function breadcrumbs(): string
    {}

    public function breadcrumbsDark(): string
    {}
}
