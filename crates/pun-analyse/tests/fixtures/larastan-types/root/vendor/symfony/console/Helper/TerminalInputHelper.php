<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

/**
 * TerminalInputHelper stops Ctrl-C and similar signals from leaving the terminal in
 * an unusable state if its settings have been modified when reading user input.
 * This can be an issue on non-Windows platforms.
 *
 * Usage:
 *
 *     $inputHelper = new TerminalInputHelper($inputStream);
 *
 *     ...change terminal settings
 *
 *     // Wait for input before all input reads
 *     $inputHelper->waitForInput();
 *
 *     ...read input
 *
 *     // Call finish to restore terminal settings and signal handlers
 *     $inputHelper->finish()
 *
 * @internal
 */
final class TerminalInputHelper
{
    /** @var resource */
    private $inputStream;
    private bool $isStdin;
    private string $initialState = '';
    private int $signalToKill = 0;
    private array $signalHandlers = [];
    private array $targetSignals = [];
    private bool $withStty;

    /**
     * @param resource $inputStream
     *
     * @throws \RuntimeException If unable to read terminal settings
     */
    public function __construct($inputStream, bool $withStty = true)
    {}

    /**
     * Waits for input.
     */
    public function waitForInput(): void
    {}

    /**
     * Restores terminal state and signal handlers.
     */
    public function finish(): void
    {}

    private function createSignalHandlers(): void
    {}

    private function checkForKillSignal(): void
    {}
}
