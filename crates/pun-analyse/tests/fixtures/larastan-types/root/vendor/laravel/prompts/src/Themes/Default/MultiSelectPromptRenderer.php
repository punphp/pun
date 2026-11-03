<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\MultiSelectPrompt;
use Laravel\Prompts\Themes\Contracts\Scrolling;

class MultiSelectPromptRenderer extends Renderer implements Scrolling
{
    use Concerns\DrawsBoxes;
    use Concerns\DrawsScrollbars;

    /**
     * Render the multiselect prompt.
     */
    public function __invoke(MultiSelectPrompt $prompt): string
    {}

    /**
     * Render the options.
     */
    protected function renderOptions(MultiSelectPrompt $prompt): string
    {}

    /**
     * Render the selected options.
     */
    protected function renderSelectedOptions(MultiSelectPrompt $prompt): string
    {}

    /**
     * Render the info text.
     */
    protected function getInfoText(MultiSelectPrompt $prompt): string
    {}

    /**
     * The number of lines to reserve outside of the scrollable area.
     */
    public function reservedLines(): int
    {}
}
