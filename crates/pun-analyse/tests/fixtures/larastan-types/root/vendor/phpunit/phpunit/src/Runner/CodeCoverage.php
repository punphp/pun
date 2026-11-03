<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner;

use const PHP_OS_FAMILY;
use function assert;
use function class_exists;
use function implode;
use function is_subclass_of;
use function sprintf;
use function sys_get_temp_dir;
use DateTimeImmutable;
use PHPUnit\Event\Emitter;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Framework\TestCase;
use PHPUnit\TextUI\Configuration\CodeCoverageFilterRegistry;
use PHPUnit\TextUI\Configuration\Configuration;
use PHPUnit\TextUI\Output\Printer;
use PHPUnit\Util\Filesystem;
use ReflectionClass;
use SebastianBergmann\CodeCoverage\Driver\Driver;
use SebastianBergmann\CodeCoverage\Driver\Granularity;
use SebastianBergmann\CodeCoverage\Driver\Selector;
use SebastianBergmann\CodeCoverage\Exception as CodeCoverageException;
use SebastianBergmann\CodeCoverage\Filter;
use SebastianBergmann\CodeCoverage\Report\Facade as ReportFacade;
use SebastianBergmann\CodeCoverage\Report\Html\Colors;
use SebastianBergmann\CodeCoverage\Report\Html\CustomCssFile;
use SebastianBergmann\CodeCoverage\Report\Html\Views;
use SebastianBergmann\CodeCoverage\Report\Thresholds;
use SebastianBergmann\CodeCoverage\Serialization\Serializer;
use SebastianBergmann\CodeCoverage\StaticAnalysis\CacheWarmer;
use SebastianBergmann\CodeCoverage\Test\Target\TargetCollection;
use SebastianBergmann\CodeCoverage\Test\Target\ValidationFailure;
use SebastianBergmann\CodeCoverage\Test\TestSize;
use SebastianBergmann\CodeCoverage\Test\TestStatus;
use SebastianBergmann\CodeCoverage\Version as CodeCoverageVersion;
use SebastianBergmann\Comparator\Comparator;
use SebastianBergmann\Environment\Runtime;
use SebastianBergmann\Timer\NoActiveTimerException;
use SebastianBergmann\Timer\Timer;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @codeCoverageIgnore
 */
final class CodeCoverage
{
    private static ?self $instance                                      = null;
    private ?\SebastianBergmann\CodeCoverage\CodeCoverage $codeCoverage = null;

    /**
     * @phpstan-ignore property.internalClass
     */
    private ?Driver $driver                     = null;
    private bool $collecting                    = false;
    private ?TestCase $test                     = null;
    private ?Timer $timer                       = null;
    private bool $requireCoverageContribution   = false;
    private bool $lastTestContributedToCoverage = false;
    private bool $collectsBranchCoverage        = false;
    private bool $collectsPathCoverage          = false;
    private readonly Emitter $emitter;

    public static function instance(): self
    {}

    public function __construct(Emitter $emitter)
    {}

    public function init(Configuration $configuration, CodeCoverageFilterRegistry $codeCoverageFilterRegistry, bool $extensionRequiresCodeCoverageCollection): CodeCoverageInitializationStatus
    {}

    /**
     * @phpstan-assert-if-true !null $this->codeCoverage
     */
    public function isActive(): bool
    {}

    public function codeCoverage(): \SebastianBergmann\CodeCoverage\CodeCoverage
    {}

    /**
     * @return non-empty-string
     */
    public function driverNameAndVersion(): string
    {}

    public function start(TestCase $test): void
    {}

    public function stop(bool $append, null|false|TargetCollection $covers = null, ?TargetCollection $uses = null): void
    {}

    public function lastTestContributedToCoverage(): bool
    {}

    public function deactivate(): void
    {}

    public function collectsBranchCoverage(): bool
    {}

    public function collectsPathCoverage(): bool
    {}

    public function generateReports(Printer $printer, Configuration $configuration): void
    {}

    public function warnAboutFilesThatCouldNotBeParsed(): void
    {}

    public function warnIfFilterIsNotConfigured(CodeCoverageFilterRegistry $codeCoverageFilterRegistry, Configuration $configuration): void
    {}

    private function activate(Filter $filter, bool $branchCoverage, bool $pathCoverage, ?string $driverClass = null): void
    {}

    /**
     * @phpstan-ignore return.internalClass
     */
    private function instantiateDriver(string $driverClass, Filter $filter, Granularity $granularity): Driver
    {}

    /**
     * The directory below the operating system's path used for temporary
     * files that is used for caching information from static analysis when
     * no cache directory is configured.
     *
     * The path used for temporary files can be shared between users, for
     * instance /tmp on Linux. Another user must not be able to control the
     * cached information, so the directory is specific to the current user
     * and is only used when no other user can write to it.
     *
     * @return false|non-empty-string
     */
    private function temporaryCoverageCacheDirectory(): false|string
    {}

    private function codeCoverageGenerationStart(Printer $printer, string $format): void
    {}

    /**
     * @throws NoActiveTimerException
     */
    private function codeCoverageGenerationSucceeded(Printer $printer): void
    {}

    /**
     * @throws NoActiveTimerException
     */
    private function codeCoverageGenerationFailed(Printer $printer, CodeCoverageException $e): void
    {}

    private function timer(): Timer
    {}
}
