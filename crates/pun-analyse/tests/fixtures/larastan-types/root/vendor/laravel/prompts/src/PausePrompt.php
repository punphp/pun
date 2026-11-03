<?php

namespace Laravel\Prompts;

class PausePrompt extends Prompt
{
    /**
     * Create a new PausePrompt instance.
     */
    public function __construct(public string $message = 'Press enter to continue...')
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): bool
    {}
}
