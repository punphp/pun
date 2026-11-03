<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration\CodeCoverage;

use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Clover;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Cobertura;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Crap4j;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Html;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Jsonl;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\OpenClover;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Php;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Text;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Xml;
use PHPUnit\TextUI\XmlConfiguration\Exception;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @immutable
 */
final readonly class CodeCoverage
{
    private ?string $driver;
    private bool $pathCoverage;
    private bool $branchCoverage;
    private bool $includeUncoveredFiles;
    private bool $ignoreDeprecatedCodeUnits;
    private bool $disableCodeCoverageIgnore;
    private ?Clover $clover;
    private ?Cobertura $cobertura;
    private ?Crap4j $crap4j;
    private ?Html $html;
    private ?Jsonl $jsonl;
    private ?OpenClover $openClover;
    private ?Php $php;
    private ?Text $text;
    private ?Xml $xml;

    public function __construct(?string $driver, bool $pathCoverage, bool $branchCoverage, bool $includeUncoveredFiles, bool $ignoreDeprecatedCodeUnits, bool $disableCodeCoverageIgnore, ?Clover $clover, ?Cobertura $cobertura, ?Crap4j $crap4j, ?Html $html, ?Jsonl $jsonl, ?OpenClover $openClover, ?Php $php, ?Text $text, ?Xml $xml)
    {}

    /**
     * @phpstan-assert-if-true !null $this->driver
     */
    public function hasDriver(): bool
    {}

    /**
     * @throws Exception
     */
    public function driver(): string
    {}

    public function pathCoverage(): bool
    {}

    public function branchCoverage(): bool
    {}

    public function includeUncoveredFiles(): bool
    {}

    public function ignoreDeprecatedCodeUnits(): bool
    {}

    public function disableCodeCoverageIgnore(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->clover
     */
    public function hasClover(): bool
    {}

    /**
     * @throws Exception
     */
    public function clover(): Clover
    {}

    /**
     * @phpstan-assert-if-true !null $this->cobertura
     */
    public function hasCobertura(): bool
    {}

    /**
     * @throws Exception
     */
    public function cobertura(): Cobertura
    {}

    /**
     * @phpstan-assert-if-true !null $this->crap4j
     */
    public function hasCrap4j(): bool
    {}

    /**
     * @throws Exception
     */
    public function crap4j(): Crap4j
    {}

    /**
     * @phpstan-assert-if-true !null $this->html
     */
    public function hasHtml(): bool
    {}

    /**
     * @throws Exception
     */
    public function html(): Html
    {}

    /**
     * @phpstan-assert-if-true !null $this->jsonl
     */
    public function hasJsonl(): bool
    {}

    /**
     * @throws Exception
     */
    public function jsonl(): Jsonl
    {}

    /**
     * @phpstan-assert-if-true !null $this->openClover
     */
    public function hasOpenClover(): bool
    {}

    /**
     * @throws Exception
     */
    public function openClover(): OpenClover
    {}

    /**
     * @phpstan-assert-if-true !null $this->php
     */
    public function hasPhp(): bool
    {}

    /**
     * @throws Exception
     */
    public function php(): Php
    {}

    /**
     * @phpstan-assert-if-true !null $this->text
     */
    public function hasText(): bool
    {}

    /**
     * @throws Exception
     */
    public function text(): Text
    {}

    /**
     * @phpstan-assert-if-true !null $this->xml
     */
    public function hasXml(): bool
    {}

    /**
     * @throws Exception
     */
    public function xml(): Xml
    {}
}
