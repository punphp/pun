<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util\PHP;

use const PHP_BINARY;
use const PHP_SAPI;
use function array_keys;
use function array_merge;
use function array_values;
use function assert;
use function count;
use function explode;
use function file_get_contents;
use function file_put_contents;
use function function_exists;
use function ini_get_all;
use function is_array;
use function is_file;
use function is_resource;
use function is_string;
use function preg_match;
use function proc_open;
use function sprintf;
use function str_contains;
use function str_replace;
use function str_starts_with;
use function sys_get_temp_dir;
use function tempnam;
use function tmpfile;
use function trim;
use function unlink;
use function xdebug_is_debugger_active;
use PHPUnit\Event\Emitter;
use PHPUnit\Framework\Test;
use PHPUnit\Framework\TestRunner\ChildProcessResultProcessor;
use PHPUnit\Runner\CodeCoverage;
use SebastianBergmann\Environment\Runtime;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class JobRunner
{
    private ChildProcessResultProcessor $processor;
    private Emitter $emitter;

    public function __construct(ChildProcessResultProcessor $processor, Emitter $emitter)
    {}

    /**
     * @param non-empty-string  $processResultFile
     * @param ?non-empty-string $processResultNonce
     */
    public function runTestJob(Job $job, string $processResultFile, Test $test, ?string $processResultNonce = null): void
    {}

    /**
     * @throws PhpProcessException
     */
    public function run(Job $job): Result
    {}

    /**
     * Spawn the code of the given job as a long-running worker process whose
     * standard input remains open as a control channel.
     *
     * The job's code is written to a temporary file that is executed by the
     * worker process; in contrast to run(), the code is not piped through
     * standard input, leaving it available for the caller to send subsequent
     * commands to the worker.
     *
     * @throws PhpProcessException
     */
    public function start(Job $job): RunningJob
    {}

    /**
     * @param ?non-empty-string $temporaryFile
     *
     * @throws PhpProcessException
     */
    private function startProcess(Job $job, ?string $temporaryFile): RunningJob
    {}

    /**
     * @return non-empty-list<string>
     */
    private function buildCommand(Job $job, ?string $file): array
    {}

    /**
     * Detects INI settings that cannot be set via ini_set() (PHP_INI_SYSTEM
     * and PHP_INI_PERDIR) and whose current value differs from the value
     * configured in INI files.
     *
     * These settings must be forwarded as -d flags to child processes
     * because the @ini_set() calls in GlobalState::getIniSettingsAsString()
     * cannot change them at runtime.
     *
     * @param array<array-key, string> $alreadySet
     *
     * @return array<string, string>
     */
    private function cliIniOverrides(array $alreadySet): array
    {}

    /**
     * @param list<string> $settings
     *
     * @throws PhpProcessException
     *
     * @return list<string>
     */
    private function settingsToParameters(array $settings): array
    {}

    /**
     * Rejects "name=value" INI settings whose value contains a line-break
     * character. A newline cannot legitimately appear in a PHP INI value and
     * would, if forwarded unchanged, be parsed by the child process as a
     * directive separator — turning a single setting into an attacker-
     * controlled sequence of directives.
     *
     * Otherwise quotes the value portion only when it contains characters
     * PHP's INI parser would interpret as metacharacters: `;` starts a
     * comment, `"` is a string delimiter, `=` is the assignment operator
     * that would otherwise be parsed as starting a new directive, `$` starts
     * `${...}` variable interpolation, and `{}|&~![()^]` are operators in
     * unquoted values (a Windows short path such as `C:\Users\RUNNER~1`
     * would otherwise trigger `PHP: syntax error, unexpected '~'`).
     *
     * Quoting is avoided for plain values so that boolean keywords such as
     * `On` / `Off` keep their special INI semantics; wrapping them in quotes
     * turns them into the literal strings `"On"` / `"Off"` and breaks
     * settings like `output_buffering`.
     *
     * @throws PhpProcessException
     */
    private function processSettingValue(string $setting): string
    {}
}
