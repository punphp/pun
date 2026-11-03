<?php

namespace Laravel\Prompts;

class Clear extends Prompt
{
    /**
     * Clear the terminal.
     */
    public function prompt(): bool
    {}

    /**
     * Clear the terminal.
     */
    public function display(): void
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): bool
    {}
}
