<?php

namespace Laravel\Prompts;

use Closure;
use Laravel\Prompts\Support\Logger;
use Laravel\Prompts\Themes\Default\Concerns\InteractsWithStrings;
use RuntimeException;

class Task extends Prompt
{
    use InteractsWithStrings;

    /**
     * The minimum width for the longest line calculation.
     */
    protected int $minWidth = 0;

    /**
     * How long to wait between rendering each frame.
     */
    public int $interval = 100;

    /**
     * The number of times the task has been rendered.
     */
    public int $count = 0;

    /**
     * Whether the task can only be rendered once.
     */
    public bool $static = false;

    /**
     * The process ID after forking.
     */
    protected int $pid;

    /**
     * The socket for IPC communication.
     *
     * @var resource|null
     */
    protected $socket;

    /**
     * Pre-wrapped log lines for the scrolling output area.
     *
     * @var array<int, string>
     */
    public array $logs = [];

    /**
     * Stable status messages (success, warning, error).
     *
     * @var list<array{type: string, message: string}>
     */
    public array $stableMessages = [];

    /**
     * The maximum number of stable messages to display.
     */
    public int $maxStableMessages = 10;

    /**
     * The identifier for the task.
     */
    public string $identifier = '';

    /**
     * Whether the task has finished.
     */
    public bool $finished = false;

    /**
     * Buffer for incomplete lines from non-blocking socket reads.
     */
    protected string $buffer = '';

    /**
     * The log index where the current partial started, or null if not streaming.
     */
    protected ?int $partialStartIndex = null;

    /**
     * Create a new Task instance.
     */
    public function __construct(
        public string $label = '',
        public int $limit = 10,
        public bool $keepSummary = false,
        public ?string $subLabel = null,
    ) {}

    /**
     * Render the task and execute the callback.
     *
     * @template TReturn of mixed
     *
     * @param  Closure(Logger): TReturn  $callback
     * @return TReturn
     */
    public function run(Closure $callback): mixed
    {}

    /**
     * Receive and process messages from the parent process.
     *
     * @param  resource  $socket
     */
    protected function receiveMessages($socket): void
    {}

    /**
     * Wrap a log line and append to the ring buffer, trimming to the limit.
     */
    protected function addLogLines(string $line): void
    {}

    /**
     * Replace the in-progress partial lines with the full accumulated text.
     */
    protected function replacePartialLines(string $text): void
    {}

    /**
     * Recompute the stable-message budget based on the current sub-label state.
     */
    protected function recalculateMaxStableMessages(): void
    {}

    /**
     * Reset the terminal.
     */
    protected function resetTerminal(bool $originalAsync, bool $success = true, callable|int $originalSignalHandler = SIG_DFL): void
    {}

    /**
     * Render a static version of the task.
     *
     * @template TReturn of mixed
     *
     * @param  Closure(Logger): TReturn  $callback
     * @return TReturn
     */
    protected function renderStatically(Closure $callback): mixed
    {}

    /**
     * Print a single-line completion indicator after the task has finished.
     */
    protected function printCompletionLine(): void
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
     * Clear the lines rendered by the task.
     */
    protected function eraseRenderedLines(): void
    {}

    /**
     * Clean up after the task.
     */
    public function __destruct()
    {}
}
