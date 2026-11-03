<?php

namespace Laravel\Prompts\Concerns;

trait Erase
{
    /**
     * Erase the given number of lines downwards from the cursor position.
     */
    public function eraseLines(int $count): void
    {}

    /**
     * Erase from cursor until end of screen.
     */
    public function eraseDown(): void
    {}
}
