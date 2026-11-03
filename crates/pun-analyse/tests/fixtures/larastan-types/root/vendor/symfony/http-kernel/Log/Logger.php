<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpKernel\Log;

use Psr\Log\AbstractLogger;
use Psr\Log\InvalidArgumentException;
use Psr\Log\LogLevel;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\RequestStack;

/**
 * Minimalist PSR-3 logger designed to write in stderr or any other stream.
 *
 * @author Kévin Dunglas <dunglas@gmail.com>
 */
class Logger extends AbstractLogger implements DebugLoggerInterface
{
    private const LEVELS = [
        LogLevel::DEBUG => 0,
        LogLevel::INFO => 1,
        LogLevel::NOTICE => 2,
        LogLevel::WARNING => 3,
        LogLevel::ERROR => 4,
        LogLevel::CRITICAL => 5,
        LogLevel::ALERT => 6,
        LogLevel::EMERGENCY => 7,
    ];
    private const PRIORITIES = [
        LogLevel::DEBUG => 100,
        LogLevel::INFO => 200,
        LogLevel::NOTICE => 250,
        LogLevel::WARNING => 300,
        LogLevel::ERROR => 400,
        LogLevel::CRITICAL => 500,
        LogLevel::ALERT => 550,
        LogLevel::EMERGENCY => 600,
    ];

    private int $minLevelIndex;
    private \Closure $formatter;
    private bool $debug = false;
    private array $logs = [];
    private array $errorCount = [];

    /** @var resource|null */
    private $handle;

    /**
     * @param string|resource|null $output
     */
    public function __construct(?string $minLevel = null, $output = null, ?callable $formatter = null, private readonly ?RequestStack $requestStack = null, bool $debug = false)
    {}

    public function enableDebug(): void
    {}

    public function log($level, $message, array $context = []): void
    {}

    public function getLogs(?Request $request = null): array
    {}

    public function countErrors(?Request $request = null): int
    {}

    public function clear(): void
    {}

    private function format(string $level, string $message, array $context, bool $prefixDate = true): string
    {}

    private function record($level, $message, array $context): void
    {}
}
