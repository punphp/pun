<?php

declare(strict_types=1);

namespace Termwind;

use Closure;
use Symfony\Component\Console\Output\OutputInterface;
use Termwind\Repositories\Styles as StyleRepository;
use Termwind\ValueObjects\Style;
use Termwind\ValueObjects\Styles;

if (! function_exists('Termwind\renderUsing')) {
    /**
     * Sets the renderer implementation.
     */
    function renderUsing(?OutputInterface $renderer): void
    {}
}

if (! function_exists('Termwind\style')) {
    /**
     * Creates a new style.
     *
     * @param  (Closure(Styles $renderable, string|int ...$arguments): Styles)|null  $callback
     */
    function style(string $name, ?Closure $callback = null): Style
    {}
}

if (! function_exists('Termwind\render')) {
    /**
     * Render HTML to the terminal.
     *
     * @param  int<0, 511>  $options
     */
    function render(string $html, int $options = OutputInterface::OUTPUT_NORMAL): void
    {}
}

if (! function_exists('Termwind\parse')) {
    /**
     * Parse HTML to a string that can be rendered in the terminal.
     */
    function parse(string $html): string
    {}
}

if (! function_exists('Termwind\terminal')) {
    /**
     * Returns a Terminal instance.
     */
    function terminal(): Terminal
    {}
}

if (! function_exists('Termwind\ask')) {
    /**
     * Renders a prompt to the user.
     *
     * @param  iterable<array-key, string>|null  $autocomplete
     */
    function ask(string $question, ?iterable $autocomplete = null): mixed
    {}
}
