<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report;

use PHPUnit\TextUI\Configuration\Directory;
use PHPUnit\TextUI\Configuration\NoCustomCssFileException;
use PHPUnit\TextUI\Configuration\NoHtmlCoverageTargetException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 */
final readonly class Html
{
    private ?Directory $target;
    private bool $classView;
    private bool $fileView;

    /**
     * @var non-negative-int
     */
    private int $lowUpperBound;

    /**
     * @var non-negative-int
     */
    private int $highLowerBound;

    /**
     * @var non-empty-string
     */
    private string $colorSuccessLow;

    /**
     * @var non-empty-string
     */
    private string $colorSuccessLowDark;

    /**
     * @var non-empty-string
     */
    private string $colorSuccessMedium;

    /**
     * @var non-empty-string
     */
    private string $colorSuccessMediumDark;

    /**
     * @var non-empty-string
     */
    private string $colorSuccessHigh;

    /**
     * @var non-empty-string
     */
    private string $colorSuccessHighDark;

    /**
     * @var non-empty-string
     */
    private string $colorSuccessBar;

    /**
     * @var non-empty-string
     */
    private string $colorSuccessBarDark;

    /**
     * @var non-empty-string
     */
    private string $colorWarning;

    /**
     * @var non-empty-string
     */
    private string $colorWarningDark;

    /**
     * @var non-empty-string
     */
    private string $colorWarningBar;

    /**
     * @var non-empty-string
     */
    private string $colorWarningBarDark;

    /**
     * @var non-empty-string
     */
    private string $colorDanger;

    /**
     * @var non-empty-string
     */
    private string $colorDangerDark;

    /**
     * @var non-empty-string
     */
    private string $colorDangerBar;

    /**
     * @var non-empty-string
     */
    private string $colorDangerBarDark;

    /**
     * @var non-empty-string
     */
    private string $colorBreadcrumbs;

    /**
     * @var non-empty-string
     */
    private string $colorBreadcrumbsDark;

    /**
     * @var ?non-empty-string
     */
    private ?string $customCssFile;

    /**
     * @param non-negative-int  $lowUpperBound
     * @param non-negative-int  $highLowerBound
     * @param non-empty-string  $colorSuccessLow
     * @param non-empty-string  $colorSuccessLowDark
     * @param non-empty-string  $colorSuccessMedium
     * @param non-empty-string  $colorSuccessMediumDark
     * @param non-empty-string  $colorSuccessHigh
     * @param non-empty-string  $colorSuccessHighDark
     * @param non-empty-string  $colorSuccessBar
     * @param non-empty-string  $colorSuccessBarDark
     * @param non-empty-string  $colorWarning
     * @param non-empty-string  $colorWarningDark
     * @param non-empty-string  $colorWarningBar
     * @param non-empty-string  $colorWarningBarDark
     * @param non-empty-string  $colorDanger
     * @param non-empty-string  $colorDangerDark
     * @param non-empty-string  $colorDangerBar
     * @param non-empty-string  $colorDangerBarDark
     * @param non-empty-string  $colorBreadcrumbs
     * @param non-empty-string  $colorBreadcrumbsDark
     * @param ?non-empty-string $customCssFile
     */
    public function __construct(?Directory $target, bool $classView, bool $fileView, int $lowUpperBound, int $highLowerBound, string $colorSuccessLow, string $colorSuccessLowDark, string $colorSuccessMedium, string $colorSuccessMediumDark, string $colorSuccessHigh, string $colorSuccessHighDark, string $colorSuccessBar, string $colorSuccessBarDark, string $colorWarning, string $colorWarningDark, string $colorWarningBar, string $colorWarningBarDark, string $colorDanger, string $colorDangerDark, string $colorDangerBar, string $colorDangerBarDark, string $colorBreadcrumbs, string $colorBreadcrumbsDark, ?string $customCssFile)
    {}

    /**
     * @phpstan-assert-if-true !null $this->target
     */
    public function hasTarget(): bool
    {}

    /**
     * @throws NoHtmlCoverageTargetException
     */
    public function target(): Directory
    {}

    public function classView(): bool
    {}

    public function fileView(): bool
    {}

    /**
     * @return non-negative-int
     */
    public function lowUpperBound(): int
    {}

    /**
     * @return non-negative-int
     */
    public function highLowerBound(): int
    {}

    /**
     * @return non-empty-string
     */
    public function colorSuccessLow(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorSuccessLowDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorSuccessMedium(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorSuccessMediumDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorSuccessHigh(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorSuccessHighDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorSuccessBar(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorSuccessBarDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorWarning(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorWarningDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorWarningBar(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorWarningBarDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorDanger(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorDangerDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorDangerBar(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorDangerBarDark(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorBreadcrumbs(): string
    {}

    /**
     * @return non-empty-string
     */
    public function colorBreadcrumbsDark(): string
    {}

    /**
     * @phpstan-assert-if-true !null $this->customCssFile
     */
    public function hasCustomCssFile(): bool
    {}

    /**
     * @throws NoCustomCssFileException
     *
     * @return non-empty-string
     */
    public function customCssFile(): string
    {}
}
