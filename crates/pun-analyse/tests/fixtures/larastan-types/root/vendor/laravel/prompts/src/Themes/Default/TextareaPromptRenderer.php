<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\TextareaPrompt;
use Laravel\Prompts\Themes\Contracts\Scrolling;

class TextareaPromptRenderer extends Renderer implements Scrolling
{
    use Concerns\DrawsBoxes;
    use Concerns\DrawsScrollbars;

    /**
     * Render the textarea prompt.
     */
    public function __invoke(TextareaPrompt $prompt): string
    {}

    /**
     * Render the text in the prompt.
     */
    protected function renderText(TextareaPrompt $prompt): string
    {}

    /**
     * The number of lines to reserve outside of the scrollable area.
     */
    public function reservedLines(): int
    {}
}
