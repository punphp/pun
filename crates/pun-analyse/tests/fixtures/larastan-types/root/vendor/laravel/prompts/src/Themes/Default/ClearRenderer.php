<?php

namespace Laravel\Prompts\Themes\Default;

class ClearRenderer extends Renderer
{
    /**
     * Clear the terminal.
     */
    public function __invoke(): string
    {}
}
