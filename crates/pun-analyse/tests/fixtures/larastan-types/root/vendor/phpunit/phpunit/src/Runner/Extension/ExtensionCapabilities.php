<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Extension;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class ExtensionCapabilities
{
    private bool $requiresCodeCoverageCollection;
    private bool $replacesOutput;
    private bool $replacesProgressOutput;
    private bool $replacesResultOutput;

    public static function from(bool $requiresCodeCoverageCollection, bool $replacesOutput, bool $replacesProgressOutput, bool $replacesResultOutput): self
    {}

    /**
     * Capabilities of a test run that does not bootstrap any extension.
     */
    public static function none(): self
    {}

    private function __construct(bool $requiresCodeCoverageCollection, bool $replacesOutput, bool $replacesProgressOutput, bool $replacesResultOutput)
    {}

    public function requiresCodeCoverageCollection(): bool
    {}

    public function replacesOutput(): bool
    {}

    public function replacesProgressOutput(): bool
    {}

    public function replacesResultOutput(): bool
    {}
}
