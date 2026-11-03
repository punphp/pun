<?php

namespace Laravel\Prompts;

use Illuminate\Support\Collection;

class Grid extends Prompt
{
    /**
     * The grid items.
     *
     * @var array<int, string>
     */
    public array $items;

    /**
     * The maximum width of the grid.
     */
    public int $maxWidth;

    /**
     * Create a new Grid instance.
     *
     * @param  array<int, string>|Collection<int, string>  $items
     */
    public function __construct(array|Collection $items = [], ?int $maxWidth = null)
    {}

    /**
     * Display the grid.
     */
    public function display(): void
    {}

    /**
     * Display the grid.
     */
    public function prompt(): bool
    {}

    /**
     * Get the value of the prompt.
     */
    public function value(): bool
    {}
}
