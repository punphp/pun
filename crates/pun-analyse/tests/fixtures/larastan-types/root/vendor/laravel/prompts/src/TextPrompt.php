<?php

namespace Laravel\Prompts;

use Closure;

class TextPrompt extends Prompt
{
    use Concerns\TypedValue;

    /**
     * Create a new TextPrompt instance.
     */
    public function __construct(
        public string $label,
        public string $placeholder = '',
        public string $default = '',
        public bool|string $required = false,
        public mixed $validate = null,
        public string $hint = '',
        public ?Closure $transform = null,
    ) {}

    /**
     * Get the entered value with a virtual cursor.
     */
    public function valueWithCursor(int $maxWidth): string
    {}
}
