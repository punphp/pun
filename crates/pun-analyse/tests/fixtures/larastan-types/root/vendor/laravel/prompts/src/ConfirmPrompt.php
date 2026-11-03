<?php

namespace Laravel\Prompts;

use Closure;

class ConfirmPrompt extends Prompt
{
    /**
     * Whether the prompt has been confirmed.
     */
    public bool $confirmed;

    /**
     * Create a new ConfirmPrompt instance.
     */
    public function __construct(
        public string $label,
        public bool $default = true,
        public string $yes = 'Yes',
        public string $no = 'No',
        public bool|string $required = false,
        public mixed $validate = null,
        public string $hint = '',
        public ?Closure $transform = null,
    ) {}

    /**
     * Get the value of the prompt.
     */
    public function value(): bool
    {}

    /**
     * Get the label of the selected option.
     */
    public function label(): string
    {}
}
