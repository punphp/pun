<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\CliArguments;

use const DIRECTORY_SEPARATOR;
use function assert;
use function basename;
use function count;
use function explode;
use function getcwd;
use function is_file;
use function is_numeric;
use function max;
use function sprintf;
use function strtolower;
use PHPUnit\Event\Emitter;
use PHPUnit\Runner\TestSuiteSorter;
use PHPUnit\TextUI\Configuration\ExecutionOrderParser;
use PHPUnit\TextUI\Configuration\ExecutionOrderSource;
use PHPUnit\Util\Filesystem;
use SebastianBergmann\CliParser\Exception as CliParserException;
use SebastianBergmann\CliParser\Parser as CliParser;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Builder
{
    /**
     * @var non-empty-list<non-empty-string>
     */
    private const array LONG_OPTIONS = [
        'all',
        'atleast-version=',
        'bootstrap=',
        'cache-result',
        'do-not-cache-result',
        'record-test-run-history',
        'do-not-record-test-run-history',
        'cache-test-index',
        'do-not-cache-test-index',
        'cache-directory=',
        'check-version',
        'check-php-configuration',
        'warn-when-php-is-not-configured-for-development',
        'do-not-warn-when-php-is-not-configured-for-development',
        'colors==',
        'columns=',
        'configuration=',
        'warm-coverage-cache',
        'coverage-filter=',
        'coverage-clover=',
        'coverage-cobertura=',
        'coverage-crap4j=',
        'coverage-html=',
        'without-class-view',
        'without-file-view',
        'coverage-jsonl=',
        'coverage-openclover=',
        'coverage-php=',
        'coverage-text==',
        'only-summary-for-coverage-text',
        'show-uncovered-for-coverage-text',
        'coverage-xml=',
        'exclude-source-from-xml-coverage',
        'path-coverage',
        'branch-coverage',
        'disallow-test-output',
        'display-all-issues',
        'display-incomplete',
        'display-skipped',
        'display-deprecations',
        'display-phpunit-deprecations',
        'display-phpunit-notices',
        'display-errors',
        'display-notices',
        'display-warnings',
        'default-time-limit=',
        'diff-context=',
        'enforce-time-limit',
        'exclude-group=',
        'filter=',
        'exclude-filter=',
        'generate-baseline=',
        'use-baseline=',
        'ignore-baseline',
        'generate-configuration',
        'globals-backup',
        'group=',
        'covers=',
        'uses=',
        'requires-php-extension=',
        'help',
        'resolve-dependencies',
        'ignore-dependencies',
        'include-path=',
        'list-groups',
        'list-suites',
        'list-test-files',
        'list-test-ids',
        'list-tests',
        'list-tests-xml=',
        'log-junit=',
        'log-otr=',
        'include-git-information',
        'log-teamcity=',
        'migrate-configuration',
        'validate-configuration',
        'no-configuration',
        'no-coverage',
        'no-logging',
        'no-extensions',
        'no-output',
        'no-progress',
        'no-results',
        'order-by=',
        'process-isolation',
        'do-not-report-useless-tests',
        'random-order',
        'random-order-seed=',
        'repeat=',
        'retry=',
        'timeout=',
        'reverse-order',
        'reverse-list',
        'static-backup',
        'stderr',
        'fail-on-all-issues',
        'fail-on-deprecation',
        'fail-on-self-deprecation',
        'fail-on-direct-deprecation',
        'fail-on-indirect-deprecation',
        'fail-on-phpunit-deprecation',
        'fail-on-phpunit-notice',
        'fail-on-phpunit-warning',
        'fail-on-empty-test-suite',
        'fail-on-incomplete',
        'fail-on-notice',
        'fail-on-risky',
        'fail-on-skipped',
        'fail-on-warning',
        'do-not-fail-on-deprecation',
        'do-not-fail-on-self-deprecation',
        'do-not-fail-on-direct-deprecation',
        'do-not-fail-on-indirect-deprecation',
        'do-not-fail-on-phpunit-deprecation',
        'do-not-fail-on-phpunit-notice',
        'do-not-fail-on-phpunit-warning',
        'do-not-fail-on-empty-test-suite',
        'do-not-fail-on-incomplete',
        'do-not-fail-on-notice',
        'do-not-fail-on-risky',
        'do-not-fail-on-skipped',
        'do-not-fail-on-warning',
        'stop-on-defect==',
        'stop-on-deprecation==',
        'stop-on-error==',
        'stop-on-failure==',
        'stop-on-incomplete==',
        'stop-on-notice==',
        'stop-on-risky==',
        'stop-on-skipped==',
        'stop-on-warning==',
        'strict-coverage',
        'require-coverage-contribution',
        'disable-coverage-ignore',
        'disable-coverage-targeting',
        'strict-global-state',
        'compact',
        'teamcity',
        'testdox',
        'testdox-summary',
        'testdox-html=',
        'testdox-text=',
        'test-suffix=',
        'testsuite=',
        'exclude-testsuite=',
        'test-files-file=',
        'test-id-filter-file=',
        'run-test-id=',
        'log-events-text=',
        'log-events-verbose-text=',
        'version',
        'debug',
        'with-telemetry',
        'extension=',
    ];

    private const string SHORT_OPTIONS = 'd:c:h';

    /**
     * @var list<array{non-empty-string, non-empty-string}>
     */
    private const array CONFLICTING_OPTIONS = [
        ['--cache-result', '--do-not-cache-result'],
        ['--record-test-run-history', '--do-not-record-test-run-history'],
        ['--cache-result', '--do-not-record-test-run-history'],
        ['--record-test-run-history', '--do-not-cache-result'],
        ['--cache-test-index', '--do-not-cache-test-index'],
        ['--warn-when-php-is-not-configured-for-development', '--do-not-warn-when-php-is-not-configured-for-development'],
        ['--fail-on-deprecation', '--do-not-fail-on-deprecation'],
        ['--fail-on-self-deprecation', '--do-not-fail-on-self-deprecation'],
        ['--fail-on-direct-deprecation', '--do-not-fail-on-direct-deprecation'],
        ['--fail-on-indirect-deprecation', '--do-not-fail-on-indirect-deprecation'],
        ['--fail-on-phpunit-deprecation', '--do-not-fail-on-phpunit-deprecation'],
        ['--fail-on-phpunit-notice', '--do-not-fail-on-phpunit-notice'],
        ['--fail-on-phpunit-warning', '--do-not-fail-on-phpunit-warning'],
        ['--fail-on-empty-test-suite', '--do-not-fail-on-empty-test-suite'],
        ['--fail-on-incomplete', '--do-not-fail-on-incomplete'],
        ['--fail-on-notice', '--do-not-fail-on-notice'],
        ['--fail-on-risky', '--do-not-fail-on-risky'],
        ['--fail-on-skipped', '--do-not-fail-on-skipped'],
        ['--fail-on-warning', '--do-not-fail-on-warning'],
        ['--resolve-dependencies', '--ignore-dependencies'],
        ['--random-order', '--reverse-order'],
        ['--repeat', '--retry'],
        ['--generate-baseline', '--ignore-baseline'],
        ['--generate-baseline', '--use-baseline'],
        ['--no-output', '--compact'],
        ['--no-output', '--teamcity'],
        ['--no-output', '--testdox'],
        ['--no-output', '--testdox-summary'],
        ['--no-output', '--debug'],
        ['--compact', '--teamcity'],
        ['--compact', '--testdox'],
        ['--compact', '--testdox-summary'],
        ['--compact', '--debug'],
    ];

    /**
     * @var list<non-empty-string>
     */
    private const array COMMAND_OPTIONS = [
        '--atleast-version',
        '--check-php-configuration',
        '--check-version',
        '--generate-configuration',
        '--help',
        '--list-groups',
        '--list-suites',
        '--list-test-files',
        '--list-test-ids',
        '--list-tests',
        '--list-tests-xml',
        '--migrate-configuration',
        '--validate-configuration',
        '--version',
        '--warm-coverage-cache',
    ];

    /**
     * @var array<string, non-negative-int>
     */
    private array $processed = [];
    private readonly Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @param list<string> $parameters
     *
     * @throws Exception
     */
    public function fromParameters(array $parameters): Configuration
    {}

    private function markProcessed(string $option): void
    {}

    private function warnAboutConflictingOptions(): void
    {}

    /**
     * @return positive-int
     */
    private function parseStopOnValue(?string $value): int
    {}

    /**
     * @throws Exception
     *
     * @return non-empty-string
     */
    private function requireNonEmptyValue(?string $value, string $option): string
    {}

    /**
     * @throws Exception
     *
     * @return positive-int
     */
    private function requirePositiveIntValue(?string $value, string $option): int
    {}
}
