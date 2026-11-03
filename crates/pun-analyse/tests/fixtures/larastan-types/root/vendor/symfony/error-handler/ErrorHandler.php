<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler;

use Psr\Log\LoggerInterface;
use Psr\Log\LogLevel;
use Symfony\Component\ErrorHandler\Error\FatalError;
use Symfony\Component\ErrorHandler\Error\MaxExecutionTimeError;
use Symfony\Component\ErrorHandler\Error\OutOfMemoryError;
use Symfony\Component\ErrorHandler\ErrorEnhancer\ClassNotFoundErrorEnhancer;
use Symfony\Component\ErrorHandler\ErrorEnhancer\ErrorEnhancerInterface;
use Symfony\Component\ErrorHandler\ErrorEnhancer\UndefinedFunctionErrorEnhancer;
use Symfony\Component\ErrorHandler\ErrorEnhancer\UndefinedMethodErrorEnhancer;
use Symfony\Component\ErrorHandler\ErrorRenderer\CliErrorRenderer;
use Symfony\Component\ErrorHandler\ErrorRenderer\HtmlErrorRenderer;
use Symfony\Component\ErrorHandler\Exception\SilencedErrorContext;

/**
 * A generic ErrorHandler for the PHP engine.
 *
 * Provides five bit fields that control how errors are handled:
 * - thrownErrors: errors thrown as \ErrorException
 * - loggedErrors: logged errors, when not @-silenced
 * - scopedErrors: errors thrown or logged with their local context
 * - tracedErrors: errors logged with their stack trace
 * - screamedErrors: never @-silenced errors
 *
 * Each error level can be logged by a dedicated PSR-3 logger object.
 * Screaming only applies to logging.
 * Throwing takes precedence over logging.
 * Uncaught exceptions are logged as E_ERROR.
 * E_DEPRECATED and E_USER_DEPRECATED levels never throw.
 * E_RECOVERABLE_ERROR and E_USER_ERROR levels always throw.
 * Non catchable errors that can be detected at shutdown time are logged when the scream bit field allows so.
 * As errors have a performance cost, repeated errors are all logged, so that the developer
 * can see them and weight them as more important to fix than others of the same level.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Grégoire Pineau <lyrixx@lyrixx.info>
 *
 * @final
 */
class ErrorHandler
{
    private array $levels = [
        \E_DEPRECATED => 'Deprecated',
        \E_USER_DEPRECATED => 'User Deprecated',
        \E_NOTICE => 'Notice',
        \E_USER_NOTICE => 'User Notice',
        \E_WARNING => 'Warning',
        \E_USER_WARNING => 'User Warning',
        \E_COMPILE_WARNING => 'Compile Warning',
        \E_CORE_WARNING => 'Core Warning',
        \E_USER_ERROR => 'User Error',
        \E_RECOVERABLE_ERROR => 'Catchable Fatal Error',
        \E_COMPILE_ERROR => 'Compile Error',
        \E_PARSE => 'Parse Error',
        \E_ERROR => 'Error',
        \E_CORE_ERROR => 'Core Error',
    ];

    private array $loggers = [
        \E_DEPRECATED => [null, LogLevel::INFO],
        \E_USER_DEPRECATED => [null, LogLevel::INFO],
        \E_NOTICE => [null, LogLevel::ERROR],
        \E_USER_NOTICE => [null, LogLevel::ERROR],
        \E_WARNING => [null, LogLevel::ERROR],
        \E_USER_WARNING => [null, LogLevel::ERROR],
        \E_COMPILE_WARNING => [null, LogLevel::ERROR],
        \E_CORE_WARNING => [null, LogLevel::ERROR],
        \E_USER_ERROR => [null, LogLevel::CRITICAL],
        \E_RECOVERABLE_ERROR => [null, LogLevel::CRITICAL],
        \E_COMPILE_ERROR => [null, LogLevel::CRITICAL],
        \E_PARSE => [null, LogLevel::CRITICAL],
        \E_ERROR => [null, LogLevel::CRITICAL],
        \E_CORE_ERROR => [null, LogLevel::CRITICAL],
    ];

    private int $thrownErrors = 0x1FFF; // E_ALL - E_DEPRECATED - E_USER_DEPRECATED
    private int $scopedErrors = 0x1FFF; // E_ALL - E_DEPRECATED - E_USER_DEPRECATED
    private int $tracedErrors = 0x77FB; // E_ALL - E_STRICT - E_PARSE
    private int $screamedErrors = 0x55; // E_ERROR + E_CORE_ERROR + E_COMPILE_ERROR + E_PARSE
    private int $loggedErrors = 0;
    private \Closure $configureException;

    private bool $isRecursive = false;
    private bool $isRoot = false;
    private bool $customExceptionHandlerCalled = false;
    /** @var callable|null */
    private $exceptionHandler;
    private ?BufferingLogger $bootstrappingLogger = null;

    private static ?string $reservedMemory = null;
    private static array $silencedErrorCache = [];
    private static int $silencedErrorCount = 0;
    private static int $exitCode = 0;

    /**
     * Registers the error handler.
     */
    public static function register(?self $handler = null, bool $replace = true): self
    {}

    /**
     * Calls a function and turns any PHP error into \ErrorException.
     *
     * @param-immediately-invoked-callable $function
     *
     * @throws \ErrorException When $function(...$arguments) triggers a PHP error
     */
    public static function call(callable $function, mixed ...$arguments): mixed
    {}

    public function __construct(
        ?BufferingLogger $bootstrappingLogger = null,
        private bool $debug = false,
    ) {}

    /**
     * Sets a logger to non assigned errors levels.
     *
     * @param LoggerInterface $logger  A PSR-3 logger to put as default for the given levels
     * @param array|int|null  $levels  An array map of E_* to LogLevel::* or an integer bit field of E_* constants
     * @param bool            $replace Whether to replace or not any existing logger
     */
    public function setDefaultLogger(LoggerInterface $logger, array|int|null $levels = \E_ALL, bool $replace = false): void
    {}

    /**
     * Sets a logger for each error level.
     *
     * @param array $loggers Error levels to [LoggerInterface|null, LogLevel::*] map
     *
     * @throws \InvalidArgumentException
     */
    public function setLoggers(array $loggers): array
    {}

    public function setExceptionHandler(?callable $handler): ?callable
    {}

    /**
     * Sets the PHP error levels that throw an exception when a PHP error occurs.
     *
     * @param int  $levels  A bit field of E_* constants for thrown errors
     * @param bool $replace Replace or amend the previous value
     */
    public function throwAt(int $levels, bool $replace = false): int
    {}

    /**
     * Sets the PHP error levels for which local variables are preserved.
     *
     * @param int  $levels  A bit field of E_* constants for scoped errors
     * @param bool $replace Replace or amend the previous value
     */
    public function scopeAt(int $levels, bool $replace = false): int
    {}

    /**
     * Sets the PHP error levels for which the stack trace is preserved.
     *
     * @param int  $levels  A bit field of E_* constants for traced errors
     * @param bool $replace Replace or amend the previous value
     */
    public function traceAt(int $levels, bool $replace = false): int
    {}

    /**
     * Sets the error levels where the @-operator is ignored.
     *
     * @param int  $levels  A bit field of E_* constants for screamed errors
     * @param bool $replace Replace or amend the previous value
     */
    public function screamAt(int $levels, bool $replace = false): int
    {}

    /**
     * Re-registers as a PHP error handler if levels changed.
     */
    private function reRegister(int $prev): void
    {}

    /**
     * Handles errors by filtering then logging them according to the configured bit fields.
     *
     * @return bool Returns false when no handling happens so that the PHP engine can handle the error itself
     *
     * @throws \ErrorException When $this->thrownErrors requests so
     *
     * @internal
     */
    public function handleError(int $type, string $message, string $file, int $line): bool
    {}

    /**
     * Handles an exception by logging then forwarding it to another handler.
     *
     * @internal
     */
    public function handleException(\Throwable $exception): void
    {}

    /**
     * Shutdown registered function for handling PHP fatal errors.
     *
     * @param array|null $error An array as returned by error_get_last()
     *
     * @internal
     */
    public static function handleFatalError(?array $error = null): void
    {}

    /**
     * Renders the given exception.
     *
     * As this method is mainly called during boot where nothing is yet available,
     * the output is always either HTML or CLI depending where PHP runs.
     */
    private function renderException(\Throwable $exception): void
    {}

    public function enhanceError(\Throwable $exception): \Throwable
    {}

    /**
     * Override this method if you want to define more error enhancers.
     *
     * @return ErrorEnhancerInterface[]
     */
    protected function getErrorEnhancers(): iterable
    {}

    /**
     * Cleans the trace by removing function arguments and the frames added by the error handler and DebugClassLoader.
     */
    private function cleanTrace(array $backtrace, int $type, string &$file, int &$line, bool $throw): array
    {}

    /**
     * Parse the error message by removing the anonymous class notation
     * and using the parent class instead if possible.
     */
    private function parseAnonymousClass(string $message): string
    {}
}
