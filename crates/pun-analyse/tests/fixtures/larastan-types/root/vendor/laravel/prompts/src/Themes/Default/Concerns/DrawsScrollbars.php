<?php

namespace Laravel\Prompts\Themes\Default\Concerns;

use Illuminate\Support\Collection;

trait DrawsScrollbars
{
    /**
     * Render a scrollbar beside the visible items.
     *
     * @template T of array<int, string>|\Illuminate\Support\Collection<int, string>
     *
     * @param  T  $visible
     * @return T
     */
    protected function scrollbar(array|Collection $visible, int $firstVisible, int $height, int $total, int $width, string $color = 'cyan'): array|Collection
    {}

    /**
     * Return the position where the scrollbar "handle" should be rendered.
     */
    protected function scrollPosition(int $firstVisible, int $height, int $total): int
    {}
}
