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

use DateTimeImmutable;
use SebastianBergmann\CodeCoverage\CodeCoverage;
use SebastianBergmann\CodeCoverage\Node\Builder;
use SebastianBergmann\CodeCoverage\Node\Directory;
use SebastianBergmann\CodeCoverage\Report\Html\Colors;
use SebastianBergmann\CodeCoverage\Report\Html\CustomCssFile;
use SebastianBergmann\CodeCoverage\Report\Html\Facade as HtmlFacade;
use SebastianBergmann\CodeCoverage\Report\Html\Views;
use SebastianBergmann\CodeCoverage\Report\Jsonl\Facade as JsonlFacade;
use SebastianBergmann\CodeCoverage\Report\Xml\Facade as XmlFacade;
use SebastianBergmann\CodeCoverage\Serialization\Serializer;
use SebastianBergmann\CodeCoverage\StaticAnalysis\FileAnalyser;
use SebastianBergmann\CodeCoverage\StaticAnalysis\ParsingSourceAnalyser;
use SebastianBergmann\CodeCoverage\Util\Filesystem;
use SebastianBergmann\Environment\Runtime;

/**
 * @phpstan-import-type SerializedCoverage from Serializer
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Facade
{
    private Directory $report;

    /**
     * @var array<non-empty-string, array{size: string, status: string, time: float}>
     */
    private array $testResults;

    public static function fromObject(CodeCoverage $codeCoverage): self
    {}

    /**
     * @param SerializedCoverage $serializedCoverage
     */
    public static function fromSerializedData(array $serializedCoverage): self
    {}

    /**
     * @param array<non-empty-string, array{size: string, status: string, time: float}> $testResults
     */
    private function __construct(Directory $report, array $testResults)
    {}

    /**
     * @param non-empty-string $target
     */
    public function renderHtml(string $target, string $generator = '', ?Colors $colors = null, ?Thresholds $thresholds = null, ?CustomCssFile $customCssFile = null, Views $views = Views::FileViewAndClassView): void
    {}

    /**
     * @param non-empty-string $target
     */
    public function renderXml(string $target, bool $includeSource = true, ?Runtime $runtime = null, ?DateTimeImmutable $buildDate = null, ?string $phpUnitVersion = null, ?string $coverageVersion = null, ?string $driverExtensionName = null, ?string $driverExtensionVersion = null): void
    {}

    /**
     * @param non-empty-string $target
     */
    public function renderJsonl(string $target): void
    {}

    /**
     * @param non-empty-string      $target
     * @param null|non-empty-string $name
     */
    public function renderClover(string $target, ?string $name = null): void
    {}

    /**
     * @param non-empty-string      $target
     * @param null|non-empty-string $name
     */
    public function renderOpenClover(string $target, ?string $name = null): void
    {}

    /**
     * @param non-empty-string $target
     */
    public function renderCobertura(string $target): void
    {}

    /**
     * @param non-empty-string      $target
     * @param null|non-empty-string $name
     */
    public function renderCrap4j(string $target, int $threshold = 30, ?string $name = null): void
    {}

    public function summary(): Summary
    {}

    /**
     * @param ?non-empty-string $target
     */
    public function renderText(?string $target, ?Thresholds $thresholds = null, bool $showUncoveredFiles = false, bool $showOnlySummary = false, bool $showColors = false): string
    {}

    /**
     * @param SerializedCoverage $serializedCoverage
     */
    private static function buildReport(array $serializedCoverage): Directory
    {}
}
