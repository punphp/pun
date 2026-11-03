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

use const DIRECTORY_SEPARATOR;
use const PHP_EOL;
use const PHP_VERSION;
use function assert;
use function defined;
use function dirname;
use function explode;
use function is_numeric;
use function max;
use function preg_match;
use function realpath;
use function sprintf;
use function str_contains;
use function str_starts_with;
use function strlen;
use function strtolower;
use function substr;
use function trim;
use DOMDocument;
use DOMElement;
use DOMNode;
use DOMNodeList;
use DOMXPath;
use PHPUnit\Event\Emitter;
use PHPUnit\Runner\TestSuiteSorter;
use PHPUnit\Runner\Version;
use PHPUnit\TextUI\Configuration\Configuration;
use PHPUnit\TextUI\Configuration\Constant;
use PHPUnit\TextUI\Configuration\ConstantCollection;
use PHPUnit\TextUI\Configuration\Directory;
use PHPUnit\TextUI\Configuration\DirectoryCollection;
use PHPUnit\TextUI\Configuration\ExecutionOrderParser;
use PHPUnit\TextUI\Configuration\ExecutionOrderSource;
use PHPUnit\TextUI\Configuration\ExtensionBootstrap;
use PHPUnit\TextUI\Configuration\ExtensionBootstrapCollection;
use PHPUnit\TextUI\Configuration\File;
use PHPUnit\TextUI\Configuration\FileCollection;
use PHPUnit\TextUI\Configuration\FilterDirectory;
use PHPUnit\TextUI\Configuration\FilterDirectoryCollection;
use PHPUnit\TextUI\Configuration\FilterFile;
use PHPUnit\TextUI\Configuration\FilterFileCollection;
use PHPUnit\TextUI\Configuration\Group;
use PHPUnit\TextUI\Configuration\GroupCollection;
use PHPUnit\TextUI\Configuration\IniSetting;
use PHPUnit\TextUI\Configuration\IniSettingCollection;
use PHPUnit\TextUI\Configuration\Php;
use PHPUnit\TextUI\Configuration\Source;
use PHPUnit\TextUI\Configuration\TestDirectory;
use PHPUnit\TextUI\Configuration\TestDirectoryCollection;
use PHPUnit\TextUI\Configuration\TestFile;
use PHPUnit\TextUI\Configuration\TestFileCollection;
use PHPUnit\TextUI\Configuration\TestSuite as TestSuiteConfiguration;
use PHPUnit\TextUI\Configuration\TestSuiteCollection;
use PHPUnit\TextUI\Configuration\Variable;
use PHPUnit\TextUI\Configuration\VariableCollection;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\CodeCoverage;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Clover;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Cobertura;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Crap4j;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Html as CodeCoverageHtml;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Jsonl;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\OpenClover;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Php as CodeCoveragePhp;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Text as CodeCoverageText;
use PHPUnit\TextUI\XmlConfiguration\CodeCoverage\Report\Xml as CodeCoverageXml;
use PHPUnit\TextUI\XmlConfiguration\Logging\Junit;
use PHPUnit\TextUI\XmlConfiguration\Logging\Logging;
use PHPUnit\TextUI\XmlConfiguration\Logging\Otr;
use PHPUnit\TextUI\XmlConfiguration\Logging\TeamCity;
use PHPUnit\TextUI\XmlConfiguration\Logging\TestDox\Html as TestDoxHtml;
use PHPUnit\TextUI\XmlConfiguration\Logging\TestDox\Text as TestDoxText;
use PHPUnit\Util\VersionComparisonOperator;
use PHPUnit\Util\Xml\Loader as XmlLoader;
use PHPUnit\Util\Xml\XmlException;
use SebastianBergmann\CodeCoverage\Report\Html\Colors;
use SebastianBergmann\CodeCoverage\Report\Thresholds;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Loader
{
    private Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @throws Exception
     */
    public function load(string $filename): LoadedFromFileConfiguration
    {}

    private function logging(string $filename, DOMXPath $xpath): Logging
    {}

    private function extensions(DOMXPath $xpath): ExtensionBootstrapCollection
    {}

    /**
     * @return non-empty-string
     */
    private function toAbsolutePath(string $filename, string $path): string
    {}

    private function source(string $filename, DOMXPath $xpath): Source
    {}

    private function codeCoverage(string $filename, DOMXPath $xpath): CodeCoverage
    {}

    private function booleanFromString(string $value, bool $default): bool
    {}

    private function valueFromString(string $value): bool|string
    {}

    private function readFilterDirectories(string $filename, DOMXPath $xpath, string $query): FilterDirectoryCollection
    {}

    private function readFilterFiles(string $filename, DOMXPath $xpath, string $query): FilterFileCollection
    {}

    private function groups(DOMXPath $xpath): Groups
    {}

    private function parseRecordTestRunHistoryAttribute(DOMElement $element): bool
    {}

    private function parseBooleanAttribute(DOMElement $element, string $attribute, bool $default): bool
    {}

    private function parseIntegerAttribute(DOMElement $element, string $attribute, int $default): int
    {}

    /**
     * @param non-negative-int $default
     *
     * @return non-negative-int
     */
    private function parseNonNegativeIntegerAttribute(DOMElement $element, string $attribute, int $default): int
    {}

    /**
     * @param positive-int $default
     *
     * @return positive-int
     */
    private function parsePositiveIntegerAttribute(DOMElement $element, string $attribute, int $default): int
    {}

    private function parseStringAttribute(DOMElement $element, string $attribute): ?string
    {}

    /**
     * @return null|non-empty-string
     */
    private function parseNullableNonEmptyStringAttribute(DOMElement $element, string $attribute): ?string
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-string
     */
    private function parseColorAttributeWithDefault(DOMElement $element, string $attribute, string $default): string
    {}

    /**
     * @throws Exception
     *
     * @return '!='|'<'|'<='|'<>'|'='|'=='|'>'|'>='|'eq'|'ge'|'gt'|'le'|'lt'|'ne'
     */
    private function parseVersionOperator(string $operator): string
    {}

    private function parseInteger(string $value, int $default): int
    {}

    private function php(string $filename, DOMXPath $xpath): Php
    {}

    private function phpunit(string $filename, DOMDocument $document, DOMXPath $xpath): PHPUnit
    {}

    /**
     * @return non-empty-string
     */
    private function parseColors(DOMDocument $document): string
    {}

    private function parseColumns(DOMDocument $document): int|string
    {}

    /**
     * @return array<non-empty-string, non-empty-string>
     */
    private function bootstrapForTestSuite(string $filename, DOMXPath $xpath): array
    {}

    private function testSuite(string $filename, DOMXPath $xpath): TestSuiteCollection
    {}

    /**
     * @return list<DOMElement>
     */
    private function parseTestSuiteElements(DOMXPath $xpath): array
    {}

    private function element(DOMXPath $xpath, string $element): ?DOMElement
    {}

    /**
     * @throws Exception
     */
    private function ensureConfigurationValidatesAgainstAtLeastOneSchema(DOMDocument $document, string $configurationFile, ValidationResult $validationResult): void
    {}
}
