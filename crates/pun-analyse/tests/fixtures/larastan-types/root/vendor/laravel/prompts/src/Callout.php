<?php

namespace Laravel\Prompts;

use Laravel\Prompts\Elements\ElementContract;

class Callout extends Prompt
{
    /**
     * Create a new Callout instance.
     *
     * @param  string|array<int, string|ElementContract>  $content
     */
    public function __construct(
        public string $label,
        public string|array $content,
        public ?string $type = null,
        public string $info = '',
    ) {}

    /**
     * Display the note.
     */
    public function display(): void
    {}

    /**
     * Display the callout.
     */
    public function prompt(): bool
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): bool
    {}
}
