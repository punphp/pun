<?php

namespace Laravel\Prompts;

use Closure;

class PasswordPrompt extends Prompt
{
    use Concerns\TypedValue;

    /**
     * Create a new PasswordPrompt instance.
     */
    public function __construct(
        public string $label,
        public string $placeholder = '',
        public bool|string $required = false,
        public mixed $validate = null,
        public string $hint = '',
        public ?Closure $transform = null,
    ) {}

    /**
     * Get a masked version of the entered value.
     */
    public function masked(): string
    {}

    /**
     * Get the masked value with a virtual cursor.
     */
    public function maskedWithCursor(int $maxWidth): string
    {}
}
