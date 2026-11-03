<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\PasswordPrompt;

class PasswordPromptRenderer extends Renderer
{
    use Concerns\DrawsBoxes;

    /**
     * Render the password prompt.
     */
    public function __invoke(PasswordPrompt $prompt): string
    {}
}
