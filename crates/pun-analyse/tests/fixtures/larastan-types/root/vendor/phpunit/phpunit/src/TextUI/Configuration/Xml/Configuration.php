<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration;

use PHPUnit\TextUI\Configuration\ExtensionBootstrapCollection;
use PHPUnit\TextUI\Configuration\Php;
use PHPUnit\TextUI\Configuration\Source;
use PHPUnit\TextUI\Configuration\TestSuiteCollection;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\CodeCoverage;
use PHPUnit\TextUI\XmlConfiguration\Logging\Logging;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 */
abstract readonly class Configuration
{
    private ExtensionBootstrapCollection $extensions;
    private Source $source;
    private CodeCoverage $codeCoverage;
    private Groups $groups;
    private Logging $logging;
    private Php $php;
    private PHPUnit $phpunit;
    private TestSuiteCollection $testSuite;

    public function __construct(ExtensionBootstrapCollection $extensions, Source $source, CodeCoverage $codeCoverage, Groups $groups, Logging $logging, Php $php, PHPUnit $phpunit, TestSuiteCollection $testSuite)
    {}

    public function extensions(): ExtensionBootstrapCollection
    {}

    public function source(): Source
    {}

    public function codeCoverage(): CodeCoverage
    {}

    public function groups(): Groups
    {}

    public function logging(): Logging
    {}

    public function php(): Php
    {}

    public function phpunit(): PHPUnit
    {}

    public function testSuite(): TestSuiteCollection
    {}

    /**
     * @phpstan-assert-if-true DefaultConfiguration $this
     */
    public function isDefault(): bool
    {}

    /**
     * @phpstan-assert-if-true LoadedFromFileConfiguration $this
     */
    public function wasLoadedFromFile(): bool
    {}
}
