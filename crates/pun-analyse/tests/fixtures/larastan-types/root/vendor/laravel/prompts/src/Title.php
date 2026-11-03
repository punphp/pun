<?php

namespace Laravel\Prompts;

class Title extends Prompt
{
    public function __construct(public string $title)
    {}

    /**
     * Update the title of the terminal.
     */
    public function prompt(): bool
    {}

    /**
     * Update the title of the terminal.
     */
    public function display(): void
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): bool
    {}
}
