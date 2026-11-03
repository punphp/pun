<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\SelectPrompt;
use Laravel\Prompts\Themes\Contracts\Scrolling;

class SelectPromptRenderer extends Renderer implements Scrolling
{
    use Concerns\DrawsBoxes;
    use Concerns\DrawsScrollbars;

    /**
     * Render the select prompt.
     */
    public function __invoke(SelectPrompt $prompt): string
    {}

    /**
     * Render the options.
     */
    protected function renderOptions(SelectPrompt $prompt): string
    {}

    /**
     * The number of lines to reserve outside of the scrollable area.
     */
    public function reservedLines(): int
    {}
}
