<?php

declare(strict_types=1);

namespace Termwind\ValueObjects;

use Closure;
use Termwind\Actions\StyleToMethod;
use Termwind\Exceptions\InvalidColor;

/**
 * @internal
 */
final class Style
{
    /**
     * Creates a new value object instance.
     *
     * @param  Closure(Styles $styles, string|int ...$argument): Styles  $callback
     */
    public function __construct(private Closure $callback, private string $color = '')
    {}

    /**
     * Apply the given set of styles to the styles.
     */
    public function apply(string $styles): void
    {}

    /**
     * Sets the color to the style.
     */
    public function color(string $color): void
    {}

    /**
     * Gets the color.
     */
    public function getColor(): string
    {}

    /**
     * Styles the given formatter with this style.
     */
    public function __invoke(Styles $styles, string|int ...$arguments): Styles
    {}
}
