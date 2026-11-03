<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\NumberPrompt;

class NumberPromptRenderer extends Renderer
{
    use Concerns\DrawsBoxes;

    protected string $upArrow = '▲';

    protected string $downArrow = '▼';

    /**
     * Render the number prompt.
     */
    public function __invoke(NumberPrompt $prompt): string
    {}

    protected function withArrows(NumberPrompt $prompt, int|string $value, ?string $color = null): string
    {}

    protected function getArrows(NumberPrompt $prompt, ?string $color = null): string
    {}
}
