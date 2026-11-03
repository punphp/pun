<?php

namespace Laravel\Prompts;

use Closure;
use InvalidArgumentException;
use RuntimeException;
use Throwable;

/**
 * @template TSteps of iterable<mixed>|int
 */
class Progress extends Prompt
{
    /**
     * The current progress bar item count.
     */
    public int $progress = 0;

    /**
     * The total number of steps.
     */
    public int $total = 0;

    /**
     * The original value of pcntl_async_signals
     */
    protected bool $originalAsync;

    /**
     * Create a new ProgressBar instance.
     *
     * @param  TSteps  $steps
     */
    public function __construct(public string $label, public iterable|int $steps, public string $hint = '')
    {}

    /**
     * Map over the steps while rendering the progress bar.
     *
     * @template TReturn
     *
     * @param  Closure((TSteps is int ? int : value-of<TSteps>), $this): TReturn  $callback
     * @return array<TReturn>
     *
     * @throws Throwable
     */
    public function map(Closure $callback): array
    {}

    /**
     * Start the progress bar.
     */
    public function start(): void
    {}

    /**
     * Advance the progress bar.
     */
    public function advance(int $step = 1): void
    {}

    /**
     * Finish the progress bar.
     */
    public function finish(): void
    {}

    /**
     * Force the progress bar to re-render.
     */
    public function render(): void
    {}

    /**
     * Update the label.
     */
    public function label(string $label): static
    {}

    /**
     * Update the hint.
     */
    public function hint(string $hint): static
    {}

    /**
     * Get the completion percentage.
     */
    public function percentage(): int|float
    {}

    /**
     * Disable prompting for input.
     *
     * @throws RuntimeException
     */
    public function prompt(): never
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): bool
    {}

    /**
     * Reset the signal handling.
     */
    protected function resetSignals(): void
    {}

    /**
     * Restore the cursor.
     */
    public function __destruct()
    {}
}
