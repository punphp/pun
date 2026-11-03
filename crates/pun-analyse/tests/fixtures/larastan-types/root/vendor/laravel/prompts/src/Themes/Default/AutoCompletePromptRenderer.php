<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\AutoCompletePrompt;

class AutoCompletePromptRenderer extends Renderer
{
    use Concerns\DrawsBoxes;

    /**
     * Render the text prompt.
     */
    public function __invoke(AutoCompletePrompt $prompt): string
    {}
}
