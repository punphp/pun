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

use const DIRECTORY_SEPARATOR;
use function count;
use function dirname;
use function file_get_contents;
use function is_array;
use function is_dir;
use function is_file;
use function is_writable;
use function sprintf;
use function strlen;
use function substr;
use DateTimeImmutable;
use SebastianBergmann\CodeCoverage\CodeCoverage;
use SebastianBergmann\CodeCoverage\Data\ProcessedClassType;
use SebastianBergmann\CodeCoverage\Data\ProcessedFunctionType;
use SebastianBergmann\CodeCoverage\Data\ProcessedTraitType;
use SebastianBergmann\CodeCoverage\Node\AbstractNode;
use SebastianBergmann\CodeCoverage\Node\Directory as DirectoryNode;
use SebastianBergmann\CodeCoverage\Node\File as FileNode;
use SebastianBergmann\CodeCoverage\PathExistsButIsNotDirectoryException;
use SebastianBergmann\CodeCoverage\Util\EnsuresUtf8;
use SebastianBergmann\CodeCoverage\Util\Filesystem;
use SebastianBergmann\CodeCoverage\Version;
use SebastianBergmann\CodeCoverage\WriteOperationFailedException;
use SebastianBergmann\CodeCoverage\XmlException;
use SebastianBergmann\Environment\Runtime;
use XMLWriter;

/**
 * @phpstan-import-type TestType from CodeCoverage
 *
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class Facade
{
    use EnsuresUtf8;

    public const string XML_NAMESPACE = 'https://schema.phpunit.de/coverage/1.0';
    private string $target;
    private Project $project;
    private readonly bool $includeSource;

    public function __construct(bool $includeSource = true)
    {}

    /**
     * @param non-empty-string                  $target
     * @param array<non-empty-string, TestType> $tests
     *
     * @throws XmlException
     */
    public function process(string $target, DirectoryNode $report, array $tests, ?Runtime $runtime = null, ?DateTimeImmutable $buildDate = null, ?string $phpUnitVersion = null, ?string $coverageVersion = null, ?string $driverExtensionName = null, ?string $driverExtensionVersion = null): void
    {}

    private function setBuildInformation(?Runtime $runtime, ?DateTimeImmutable $buildDate, ?string $phpUnitVersion, ?string $coverageVersion, ?string $driverExtensionName, ?string $driverExtensionVersion): void
    {}

    /**
     * @throws PathExistsButIsNotDirectoryException
     * @throws WriteOperationFailedException
     */
    private function initTargetDirectory(string $directory): void
    {}

    /**
     * @throws XmlException
     */
    private function processDirectory(DirectoryNode $directory, Node $context): void
    {}

    /**
     * @throws XmlException
     */
    private function processFile(FileNode $file, Directory $context): void
    {}

    private function processUnit(ProcessedClassType|ProcessedTraitType $unit, Report $report): void
    {}

    private function processFunction(ProcessedFunctionType $function, Report $report): void
    {}

    /**
     * @param array<non-empty-string, TestType> $tests
     */
    private function processTests(array $tests): void
    {}

    private function setTotals(AbstractNode $node, Totals $totals): void
    {}

    private function targetDirectory(): string
    {}

    private function targetFilePath(string $name): string
    {}
}
