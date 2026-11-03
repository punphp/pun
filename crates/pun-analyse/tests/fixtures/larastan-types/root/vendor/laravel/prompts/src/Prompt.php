<?php

namespace Laravel\Prompts;

use Closure;
use Laravel\Prompts\Exceptions\FormRevertedException;
use Laravel\Prompts\Output\ConsoleOutput;
use Laravel\Prompts\Support\Result;
use RuntimeException;
use Symfony\Component\Console\Output\OutputInterface;
use Throwable;

abstract class Prompt
{
    use Concerns\Colors;
    use Concerns\Cursor;
    use Concerns\Erase;
    use Concerns\Events;
    use Concerns\FakesInputOutput;
    use Concerns\Fallback;
    use Concerns\Interactivity;
    use Concerns\Themes;

    /**
     * The current state of the prompt.
     */
    public string $state = 'initial';

    /**
     * The error message from the validator.
     */
    public string $error = '';

    /**
     * The cancel message displayed when this prompt is cancelled.
     */
    public string $cancelMessage = 'Cancelled.';

    /**
     * The previously rendered frame.
     */
    protected string $prevFrame = '';

    /**
     * How many new lines were written by the last output.
     */
    protected int $newLinesWritten = 1;

    /**
     * Whether user input is required.
     */
    public bool|string $required;

    /**
     * The transformation callback.
     */
    public ?Closure $transform = null;

    /**
     * The validator callback or rules.
     */
    public mixed $validate;

    /**
     * The cancellation callback.
     */
    protected static ?Closure $cancelUsing;

    /**
     * Indicates if the prompt has been validated.
     */
    protected bool $validated = false;

    /**
     * The custom validation callback.
     */
    protected static ?Closure $validateUsing;

    /**
     * The revert handler from the StepBuilder.
     */
    protected static ?Closure $revertUsing = null;

    /**
     * The output instance.
     */
    protected static OutputInterface $output;

    /**
     * The terminal instance.
     */
    protected static Terminal $terminal;

    /**
     * Get the value of the prompt.
     */
    abstract public function value(): mixed;

    /**
     * Render the prompt and listen for input.
     */
    public function prompt(): mixed
    {}

    /**
     * Implementation of the prompt looping mechanism.
     *
     * @param  callable(string $key): ?Result  $callable
     */
    public function runLoop(callable $callable): mixed
    {}

    /**
     * Register a callback to be invoked when a user cancels a prompt.
     */
    public static function cancelUsing(?Closure $callback): void
    {}

    /**
     * How many new lines were written by the last output.
     */
    public function newLinesWritten(): int
    {}

    /**
     * Capture the number of new lines written by the last output.
     */
    protected function capturePreviousNewLines(): void
    {}

    /**
     * Set the output instance.
     */
    public static function setOutput(OutputInterface $output): void
    {}

    /**
     * Get the current output instance.
     */
    protected static function output(): OutputInterface
    {}

    /**
     * Write output directly, bypassing newline capture.
     */
    protected static function writeDirectly(string $message): void
    {}

    /**
     * Get the terminal instance.
     */
    public static function terminal(): Terminal
    {}

    /**
     * Set the custom validation callback.
     */
    public static function validateUsing(Closure $callback): void
    {}

    /**
     * Revert the prompt using the given callback.
     *
     * @internal
     */
    public static function revertUsing(Closure $callback): void
    {}

    /**
     * Clear any previous revert callback.
     *
     * @internal
     */
    public static function preventReverting(): void
    {}

    /**
     * Render the prompt.
     */
    protected function render(): void
    {}

    /**
     * Submit the prompt.
     */
    protected function submit(): void
    {}

    /**
     * Handle a key press and determine whether to continue.
     */
    private function handleKeyPress(string $key): bool
    {}

    /**
     * Transform the input.
     */
    private function transform(mixed $value): mixed
    {}

    /**
     * Get the transformed value of the prompt.
     */
    protected function transformedValue(): mixed
    {}

    /**
     * Validate the input.
     */
    private function validate(mixed $value): void
    {}

    /**
     * Determine whether the given value is invalid when the prompt is required.
     */
    protected function isInvalidWhenRequired(mixed $value): bool
    {}

    /**
     * Check whether the environment can support the prompt.
     */
    private function checkEnvironment(): void
    {}

    /**
     * Restore the cursor and terminal state.
     */
    public function __destruct()
    {}

    /**
     * Flush the global state of Prompts.
     */
    public static function flushState(): void
    {}
}
