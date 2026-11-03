<?php

namespace Laravel\Prompts\Themes\Default;

use Laravel\Prompts\Progress;

class ProgressRenderer extends Renderer
{
    use Concerns\DrawsBoxes;

    /**
     * The character to use for the progress bar.
     */
    protected string $barCharacter = '█';

    /**
     * Render the progress bar.
     *
     * @param  Progress<int|iterable<mixed>>  $progress
     */
    public function __invoke(Progress $progress): string
    {}

    /**
     * @param  Progress<int|iterable<mixed>>  $progress
     */
    protected function fractionCompleted(Progress $progress): string
    {}
}
