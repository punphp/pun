<?php

namespace Laravel\Prompts;

use Closure;
use RuntimeException;

class Spinner extends Prompt
{
    /**
     * How long to wait between rendering each frame.
     */
    public int $interval = 100;

    /**
     * The number of times the spinner has been rendered.
     */
    public int $count = 0;

    /**
     * Whether the spinner can only be rendered once.
     */
    public bool $static = false;

    /**
     * The process ID after forking.
     */
    protected int $pid;

    /**
     * Create a new Spinner instance.
     */
    public function __construct(public string $message = '')
    {}

    /**
     * Render the spinner and execute the callback.
     *
     * @template TReturn of mixed
     *
     * @param  Closure(): TReturn  $callback
     * @return TReturn
     */
    public function spin(Closure $callback): mixed
    {}

    /**
     * Reset the terminal.
     */
    protected function resetTerminal(bool $originalAsync, callable|int $originalSignalHandler = SIG_DFL): void
    {}

    /**
     * Render a static version of the spinner.
     *
     * @template TReturn of mixed
     *
     * @param  Closure(): TReturn  $callback
     * @return TReturn
     */
    protected function renderStatically(Closure $callback): mixed
    {}

    /**
     * Disable prompting for input.
     *
     * @throws RuntimeException
     */
    public function prompt(): never
    {}

    /**
     * Get the current value of the prompt.
     */
    public function value(): bool
    {}

    /**
     * Clear the lines rendered by the spinner.
     */
    protected function eraseRenderedLines(): void
    {}

    /**
     * Clean up after the spinner.
     */
    public function __destruct()
    {}
}
