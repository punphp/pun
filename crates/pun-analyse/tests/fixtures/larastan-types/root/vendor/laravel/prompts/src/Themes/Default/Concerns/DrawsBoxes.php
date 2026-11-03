<?php

namespace Laravel\Prompts\Themes\Default\Concerns;

use Laravel\Prompts\Prompt;

trait DrawsBoxes
{
    use InteractsWithStrings;

    protected int $minWidth = 60;

    /**
     * Draw a box.
     *
     * @return $this
     */
    protected function box(
        string $title,
        string $body,
        string $footer = '',
        string $color = 'gray',
        string $info = '',
    ): self {}
}
