<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 *
 * @phpstan-type DeprecationTriggers array{functions: list<non-empty-string>, methods: list<non-empty-string>, ignoreUndefinedTriggers: bool}
 */
final readonly class Source
{
    /**
     * @var non-empty-string
     */
    private ?string $baseline;
    private bool $ignoreBaseline;
    private FilterDirectoryCollection $includeDirectories;
    private FilterFileCollection $includeFiles;
    private FilterDirectoryCollection $excludeDirectories;
    private FilterFileCollection $excludeFiles;
    private bool $restrictNotices;
    private bool $restrictWarnings;
    private bool $ignoreSuppressionOfDeprecations;
    private bool $ignoreSuppressionOfPhpDeprecations;
    private bool $ignoreSuppressionOfErrors;
    private bool $ignoreSuppressionOfNotices;
    private bool $ignoreSuppressionOfPhpNotices;
    private bool $ignoreSuppressionOfWarnings;
    private bool $ignoreSuppressionOfPhpWarnings;
    private bool $ignoreSelfDeprecations;
    private bool $ignoreDirectDeprecations;
    private bool $ignoreIndirectDeprecations;
    private bool $identifyIssueTrigger;

    /**
     * @var DeprecationTriggers
     */
    private array $deprecationTriggers;

    /**
     * @var list<non-empty-string>
     */
    private array $issueTriggerResolvers;

    /**
     * @var list<non-empty-string>
     */
    private array $deprecationFilters;

    /**
     * @param ?non-empty-string      $baseline
     * @param DeprecationTriggers    $deprecationTriggers
     * @param list<non-empty-string> $issueTriggerResolvers
     * @param list<non-empty-string> $deprecationFilters
     */
    public function __construct(?string $baseline, bool $ignoreBaseline, FilterDirectoryCollection $includeDirectories, FilterFileCollection $includeFiles, FilterDirectoryCollection $excludeDirectories, FilterFileCollection $excludeFiles, bool $restrictNotices, bool $restrictWarnings, bool $ignoreSuppressionOfDeprecations, bool $ignoreSuppressionOfPhpDeprecations, bool $ignoreSuppressionOfErrors, bool $ignoreSuppressionOfNotices, bool $ignoreSuppressionOfPhpNotices, bool $ignoreSuppressionOfWarnings, bool $ignoreSuppressionOfPhpWarnings, array $deprecationTriggers, bool $ignoreSelfDeprecations, bool $ignoreDirectDeprecations, bool $ignoreIndirectDeprecations, bool $identifyIssueTrigger, array $issueTriggerResolvers = [], array $deprecationFilters = [])
    {}

    /**
     * @phpstan-assert-if-true !null $this->baseline
     */
    public function useBaseline(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->baseline
     */
    public function hasBaseline(): bool
    {}

    /**
     * @throws NoBaselineException
     *
     * @return non-empty-string
     */
    public function baseline(): string
    {}

    public function includeDirectories(): FilterDirectoryCollection
    {}

    public function includeFiles(): FilterFileCollection
    {}

    public function excludeDirectories(): FilterDirectoryCollection
    {}

    public function excludeFiles(): FilterFileCollection
    {}

    public function notEmpty(): bool
    {}

    public function restrictNotices(): bool
    {}

    public function restrictWarnings(): bool
    {}

    public function ignoreSuppressionOfDeprecations(): bool
    {}

    public function ignoreSuppressionOfPhpDeprecations(): bool
    {}

    public function ignoreSuppressionOfErrors(): bool
    {}

    public function ignoreSuppressionOfNotices(): bool
    {}

    public function ignoreSuppressionOfPhpNotices(): bool
    {}

    public function ignoreSuppressionOfWarnings(): bool
    {}

    public function ignoreSuppressionOfPhpWarnings(): bool
    {}

    /**
     * @return DeprecationTriggers
     */
    public function deprecationTriggers(): array
    {}

    public function ignoreSelfDeprecations(): bool
    {}

    public function ignoreDirectDeprecations(): bool
    {}

    public function ignoreIndirectDeprecations(): bool
    {}

    public function identifyIssueTrigger(): bool
    {}

    /**
     * @return list<non-empty-string>
     */
    public function issueTriggerResolvers(): array
    {}

    /**
     * @return list<non-empty-string>
     */
    public function deprecationFilters(): array
    {}
}
