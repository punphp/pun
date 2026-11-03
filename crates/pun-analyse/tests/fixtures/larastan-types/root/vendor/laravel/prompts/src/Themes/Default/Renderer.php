<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\Concerns\Colors;
use Laravel\Prompts\Concerns\Truncation;
use Laravel\Prompts\Prompt;

abstract class Renderer
{
    use Colors;
    use Truncation;

    /**
     * The output to be rendered.
     */
    protected string $output = '';

    /**
     * Create a new renderer instance.
     */
    public function __construct(protected Prompt $prompt)
    {}

    /**
     * Render a line of output.
     */
    protected function line(string $message): self
    {}

    /**
     * Render a new line.
     */
    protected function newLine(int $count = 1): self
    {}

    /**
     * Render a warning message.
     */
    protected function warning(string $message): self
    {}

    /**
     * Render an error message.
     */
    protected function error(string $message): self
    {}

    /**
     * Render an hint message.
     */
    protected function hint(string $message): self
    {}

    /**
     * Apply the callback if the given "value" is truthy.
     *
     * @return $this
     */
    protected function when(mixed $value, callable $callback, ?callable $default = null): self
    {}

    /**
     * Render the output with a blank line above and below.
     */
    public function __toString()
    {}
}
