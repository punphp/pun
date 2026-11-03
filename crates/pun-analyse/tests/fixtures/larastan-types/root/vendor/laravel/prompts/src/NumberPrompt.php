<?php

namespace Laravel\Prompts;

use Closure;
use RuntimeException;

class NumberPrompt extends Prompt
{
    use Concerns\TypedValue;

    /**
     * Create a new NumberPrompt instance.
     */
    public function __construct(
        public string $label,
        public string $placeholder = '',
        public string $default = '',
        public bool|string $required = false,
        public mixed $validate = null,
        public string $hint = '',
        public ?Closure $transform = null,
        public ?int $min = null,
        public ?int $max = null,
        public ?int $step = null,
    ) {}

    protected function wrapValidation(mixed $validate): callable
    {}

    /**
     * Increase the value of the prompt by the step.
     */
    protected function increaseValue(): void
    {}

    /**
     * Decrease the value of the prompt by the step.
     */
    protected function decreaseValue(): void
    {}

    public function value(): int|string
    {}

    /**
     * Get the entered value with a virtual cursor.
     */
    public function valueWithCursor(int $maxWidth): string
    {}
}
