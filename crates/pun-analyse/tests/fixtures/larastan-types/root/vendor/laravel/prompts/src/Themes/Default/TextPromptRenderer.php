<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\TextPrompt;

class TextPromptRenderer extends Renderer
{
    use Concerns\DrawsBoxes;

    /**
     * Render the text prompt.
     */
    public function __invoke(TextPrompt $prompt): string
    {}
}
