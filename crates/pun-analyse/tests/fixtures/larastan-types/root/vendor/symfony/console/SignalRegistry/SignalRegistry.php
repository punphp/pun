<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\SignalRegistry;

final class SignalRegistry
{
    /**
     * @var array<int, array<callable>>
     */
    private array $signalHandlers = [];

    /**
     * @var array<array<int, array<callable>>>
     */
    private array $stack = [];

    /**
     * @var array<int, callable|int|string>
     */
    private array $originalHandlers = [];

    public function __construct()
    {}

    public function register(int $signal, callable $signalHandler): void
    {}

    public static function isSupported(): bool
    {}

    /**
     * @internal
     */
    public function handle(int $signal): void
    {}

    /**
     * Pushes the current active handlers onto the stack and clears the active list.
     *
     * This prepares the registry for a new set of handlers within a specific scope.
     *
     * @internal
     */
    public function pushCurrentHandlers(): void
    {}

    /**
     * Restores the previous handlers from the stack, making them active.
     *
     * This also restores the original OS-level signal handler if no
     * more handlers are registered for a signal that was just popped.
     *
     * @internal
     */
    public function popPreviousHandlers(): void
    {}

    /**
     * @internal
     */
    public function scheduleAlarm(int $seconds): void
    {}
}
