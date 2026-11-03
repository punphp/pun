<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\ConfirmPrompt;

class ConfirmPromptRenderer extends Renderer
{
    use Concerns\DrawsBoxes;

    /**
     * Render the confirm prompt.
     */
    public function __invoke(ConfirmPrompt $prompt): string
    {}

    /**
     * Render the confirm prompt options.
     */
    protected function renderOptions(ConfirmPrompt $prompt): string
    {}
}
