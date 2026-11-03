<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Xml;

use DateTimeImmutable;
use SebastianBergmann\Environment\Runtime;
use XMLWriter;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class Project extends Node
{
    private readonly string $directory;

    public function __construct(XMLWriter $xmlWriter, string $directory)
    {}

    public function projectSourceDirectory(): string
    {}

    public function buildInformation(
        Runtime $runtime,
        DateTimeImmutable $buildDate,
        string $phpUnitVersion,
        string $coverageVersion,
        string $driverExtensionName,
        string $driverExtensionVersion,
    ): void {}

    public function tests(): Tests
    {}

    public function getWriter(): XMLWriter
    {}

    public function startProject(): void
    {}

    public function finalize(): void
    {}
}
