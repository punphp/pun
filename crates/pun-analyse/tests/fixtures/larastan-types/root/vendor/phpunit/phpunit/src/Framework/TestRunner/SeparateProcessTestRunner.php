<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestRunner;

use function assert;
use function bin2hex;
use function defined;
use function get_include_path;
use function getmypid;
use function hrtime;
use function random_bytes;
use function register_shutdown_function;
use function serialize;
use function sprintf;
use function sys_get_temp_dir;
use function tempnam;
use function unlink;
use function var_export;
use PHPUnit\Event\Code\TestMethodBuilder;
use PHPUnit\Event\Code\ThrowableBuilder;
use PHPUnit\Event\Emitter;
use PHPUnit\Event\NoPreviousThrowableException;
use PHPUnit\Event\TestRunner\ChildProcessReason;
use PHPUnit\Framework\AssertionFailedError;
use PHPUnit\Framework\Exception;
use PHPUnit\Framework\ProcessIsolationException;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\Api\Requirements;
use PHPUnit\Runner\CodeCoverage;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use PHPUnit\TextUI\Configuration\SourceMapper;
use PHPUnit\Util\GlobalState;
use PHPUnit\Util\PHP\Job;
use PHPUnit\Util\PHP\JobRunnerRegistry;
use ReflectionClass;
use SebastianBergmann\Template\InvalidArgumentException;
use SebastianBergmann\Template\Template;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class SeparateProcessTestRunner
{
    private static ?string $configurationFile = null;
    private static ?string $sourceMapFile     = null;
    private readonly Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @throws \PHPUnit\Runner\Exception
     * @throws \PHPUnit\Util\Exception
     * @throws Exception
     * @throws InvalidArgumentException
     * @throws NoPreviousThrowableException
     * @throws ProcessIsolationException
     */
    public function run(TestCase $test): void
    {}

    /**
     * @throws Exception
     * @throws NoPreviousThrowableException
     */
    private function testCouldNotBeRun(TestCase $test, string $message): void
    {}

    private function sourceMapFileForChildProcess(): string
    {}

    /**
     * @throws ProcessIsolationException
     */
    private function configurationFileForChildProcess(): string
    {}

    private function createTemporaryFile(): false|string
    {}

    /**
     * The configuration and the source map do not change while the test run is
     * in progress: each is written once and shared by all child processes, so
     * neither can be removed before the test run has ended.
     */
    private function createTemporaryFileSharedByChildProcesses(): false|string
    {}

    /**
     * The given value as the contents of a single-quoted string literal of
     * a template: the value is exported as a string literal of its own that
     * is concatenated with the empty strings that the quotes of the template
     * delimit, as is done for the data of the test above.
     */
    private static function stringLiteralFragment(string $value): string
    {}
}
