<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\MultiSearchPrompt;
use Laravel\Prompts\Themes\Contracts\Scrolling;

class MultiSearchPromptRenderer extends Renderer implements Scrolling
{
    use Concerns\DrawsBoxes;
    use Concerns\DrawsScrollbars;

    /**
     * Render the suggest prompt.
     */
    public function __invoke(MultiSearchPrompt $prompt): string
    {}

    /**
     * Render the value with the cursor and a search icon.
     */
    protected function valueWithCursorAndSearchIcon(MultiSearchPrompt $prompt, int $maxWidth): string
    {}

    /**
     * Render a spacer to prevent jumping when the suggestions are displayed.
     */
    protected function spaceForDropdown(MultiSearchPrompt $prompt): self
    {}

    /**
     * Render the options.
     */
    protected function renderOptions(MultiSearchPrompt $prompt): string
    {}

    /**
     * Render the selected options.
     */
    protected function renderSelectedOptions(MultiSearchPrompt $prompt): string
    {}

    /**
     * Render the info text.
     */
    protected function getInfoText(MultiSearchPrompt $prompt): string
    {}

    /**
     * The number of lines to reserve outside of the scrollable area.
     */
    public function reservedLines(): int
    {}
}
