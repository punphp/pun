<?php

declare(strict_types=1);

namespace Termwind;

use Closure;
use Symfony\Component\Console\Output\ConsoleOutput;
use Symfony\Component\Console\Output\OutputInterface;
use Termwind\Components\Element;
use Termwind\Exceptions\InvalidChild;

/**
 * @internal
 */
final class Termwind
{
    /**
     * The implementation of the output.
     */
    private static ?OutputInterface $renderer;

    /**
     * Sets the renderer implementation.
     */
    public static function renderUsing(?OutputInterface $renderer): void
    {}

    /**
     * Creates a div element instance.
     *
     * @param  array<int, Element|string>|string  $content
     * @param  array<string, mixed>  $properties
     */
    public static function div(array|string $content = '', string $styles = '', array $properties = []): Components\Div
    {}

    /**
     * Creates a paragraph element instance.
     *
     * @param  array<int, Element|string>|string  $content
     * @param  array<string, mixed>  $properties
     */
    public static function paragraph(array|string $content = '', string $styles = '', array $properties = []): Components\Paragraph
    {}

    /**
     * Creates a span element instance with the given style.
     *
     * @param  array<int, Element|string>|string  $content
     * @param  array<string, mixed>  $properties
     */
    public static function span(array|string $content = '', string $styles = '', array $properties = []): Components\Span
    {}

    /**
     * Creates an element instance with raw content.
     *
     * @param  array<int, Element|string>|string  $content
     */
    public static function raw(array|string $content = ''): Components\Raw
    {}

    /**
     * Creates an anchor element instance with the given style.
     *
     * @param  array<int, Element|string>|string  $content
     * @param  array<string, mixed>  $properties
     */
    public static function anchor(array|string $content = '', string $styles = '', array $properties = []): Components\Anchor
    {}

    /**
     * Creates an unordered list instance.
     *
     * @param  array<int, string|Element>  $content
     * @param  array<string, mixed>  $properties
     */
    public static function ul(array $content = [], string $styles = '', array $properties = []): Components\Ul
    {}

    /**
     * Creates an ordered list instance.
     *
     * @param  array<int, string|Element>  $content
     * @param  array<string, mixed>  $properties
     */
    public static function ol(array $content = [], string $styles = '', array $properties = []): Components\Ol
    {}

    /**
     * Creates a list item instance.
     *
     * @param  array<int, Element|string>|string  $content
     * @param  array<string, mixed>  $properties
     */
    public static function li(array|string $content = '', string $styles = '', array $properties = []): Components\Li
    {}

    /**
     * Creates a description list instance.
     *
     * @param  array<int, string|Element>  $content
     * @param  array<string, mixed>  $properties
     */
    public static function dl(array $content = [], string $styles = '', array $properties = []): Components\Dl
    {}

    /**
     * Creates a description term instance.
     *
     * @param  array<int, Element|string>|string  $content
     * @param  array<string, mixed>  $properties
     */
    public static function dt(array|string $content = '', string $styles = '', array $properties = []): Components\Dt
    {}

    /**
     * Creates a description details instance.
     *
     * @param  array<int, Element|string>|string  $content
     * @param  array<string, mixed>  $properties
     */
    public static function dd(array|string $content = '', string $styles = '', array $properties = []): Components\Dd
    {}

    /**
     * Creates a horizontal rule instance.
     *
     * @param  array<string, mixed>  $properties
     */
    public static function hr(string $styles = '', array $properties = []): Components\Hr
    {}

    /**
     * Creates an break line element instance.
     *
     * @param  array<string, mixed>  $properties
     */
    public static function breakLine(string $styles = '', array $properties = []): Components\BreakLine
    {}

    /**
     * Gets the current renderer instance.
     */
    public static function getRenderer(): OutputInterface
    {}

    /**
     * Convert child elements to a string.
     *
     * @param  array<int, string|Element>|string  $elements
     * @return array<int, string|Element>
     */
    private static function prepareElements($elements, string $styles = '', ?Closure $callback = null): array
    {}
}
