<?php

declare(strict_types=1);

namespace Termwind\ValueObjects;

use Closure;
use Termwind\Actions\StyleToMethod;
use Termwind\Components\Element;
use Termwind\Components\Hr;
use Termwind\Components\Li;
use Termwind\Components\Ol;
use Termwind\Components\Ul;
use Termwind\Enums\Color;
use Termwind\Exceptions\ColorNotFound;
use Termwind\Exceptions\InvalidStyle;
use Termwind\Repositories\Styles as StyleRepository;

use function Termwind\terminal;

/**
 * @internal
 */
final class Styles
{
    /**
     * Finds all the styling on a string.
     */
    public const STYLING_REGEX = "/\<[\w=#\/\;,:.&,%?-]+\>|\\e\[\d+m/";

    /** @var array<int, string> */
    private array $styles = [];

    private ?Element $element = null;

    /**
     * Creates a Style formatter instance.
     *
     * @param  array<string, mixed>  $properties
     * @param  array<string, Closure(string, array<string, string|int>, array<string, int[]>): string>  $textModifiers
     * @param  array<string, Closure(string, array<string, string|int>): string>  $styleModifiers
     * @param  string[]  $defaultStyles
     */
    final public function __construct(
        private array $properties = [
            'colors' => [],
            'options' => [],
            'isFirstChild' => false,
        ],
        private array $textModifiers = [],
        private array $styleModifiers = [],
        private array $defaultStyles = []
    ) {}

    /**
     * @return $this
     */
    public function setElement(Element $element): self
    {}

    /**
     * Gets default styles.
     *
     * @return string[]
     */
    public function defaultStyles(): array
    {}

    /**
     * Gets the element's style properties.
     *
     * @return array<string, mixed>
     */
    final public function getProperties(): array
    {}

    /**
     * Sets the element's style properties.
     *
     * @param  array<string, mixed>  $properties
     */
    public function setProperties(array $properties): self
    {}

    /**
     * Sets the styles to the element.
     */
    final public function setStyle(string $style): self
    {}

    /**
     * Checks if the element has the style.
     */
    final public function hasStyle(string $style): bool
    {}

    /**
     * Adds a style to the element.
     */
    final public function addStyle(string $style): self
    {}

    /**
     * Inherit styles from given Styles object.
     */
    final public function inheritFromStyles(self $styles): self
    {}

    /**
     * Adds a background color to the element.
     */
    final public function bg(string $color, int $variant = 0): self
    {}

    /**
     * Adds a bold style to the element.
     */
    final public function fontBold(): self
    {}

    /**
     * Removes the bold style on the element.
     */
    final public function fontNormal(): self
    {}

    /**
     * Adds a bold style to the element.
     */
    final public function strong(): self
    {}

    /**
     * Adds an italic style to the element.
     */
    final public function italic(): self
    {}

    /**
     * Adds an underline style.
     */
    final public function underline(): self
    {}

    /**
     * Adds the given margin left to the element.
     */
    final public function ml(int $margin): self
    {}

    /**
     * Adds the given margin right to the element.
     */
    final public function mr(int $margin): self
    {}

    /**
     * Adds the given margin bottom to the element.
     */
    final public function mb(int $margin): self
    {}

    /**
     * Adds the given margin top to the element.
     */
    final public function mt(int $margin): self
    {}

    /**
     * Adds the given horizontal margin to the element.
     */
    final public function mx(int $margin): self
    {}

    /**
     * Adds the given vertical margin to the element.
     */
    final public function my(int $margin): self
    {}

    /**
     * Adds the given margin to the element.
     */
    final public function m(int $margin): self
    {}

    /**
     * Adds the given padding left to the element.
     */
    final public function pl(int $padding): static
    {}

    /**
     * Adds the given padding right.
     */
    final public function pr(int $padding): static
    {}

    /**
     * Adds the given horizontal padding.
     */
    final public function px(int $padding): self
    {}

    /**
     * Adds the given padding top.
     */
    final public function pt(int $padding): static
    {}

    /**
     * Adds the given padding bottom.
     */
    final public function pb(int $padding): static
    {}

    /**
     * Adds the given vertical padding.
     */
    final public function py(int $padding): self
    {}

    /**
     * Adds the given padding.
     */
    final public function p(int $padding): self
    {}

    /**
     * Adds the given vertical margin to the childs, ignoring the first child.
     */
    final public function spaceY(int $space): self
    {}

    /**
     * Adds the given horizontal margin to the childs, ignoring the first child.
     */
    final public function spaceX(int $space): self
    {}

    /**
     * Adds a border on top of each element.
     */
    final public function borderT(int $width = 1): self
    {}

    /**
     * Adds a text alignment or color to the element.
     */
    final public function text(string $value, int $variant = 0): self
    {}

    /**
     * Truncates the text of the element.
     */
    final public function truncate(int $limit = 0, string $end = '…'): self
    {}

    /**
     * Forces the width of the element.
     */
    final public function w(int|string $width): static
    {}

    /**
     * Forces the element width to the full width of the terminal.
     */
    final public function wFull(): static
    {}

    /**
     * Removes the width set on the element.
     */
    final public function wAuto(): static
    {}

    /**
     * Defines a minimum width of an element.
     */
    final public function minW(int|string $width): static
    {}

    /**
     * Defines a maximum width of an element.
     */
    final public function maxW(int|string $width): static
    {}

    /**
     * Makes the element's content uppercase.
     */
    final public function uppercase(): self
    {}

    /**
     * Makes the element's content lowercase.
     */
    final public function lowercase(): self
    {}

    /**
     * Makes the element's content capitalize.
     */
    final public function capitalize(): self
    {}

    /**
     * Makes the element's content in snakecase.
     */
    final public function snakecase(): self
    {}

    /**
     * Makes the element's content with a line through.
     */
    final public function lineThrough(): self
    {}

    /**
     * Makes the element's content invisible.
     */
    final public function invisible(): self
    {}

    /**
     * Do not display element's content.
     */
    final public function hidden(): self
    {}

    /**
     * Makes a line break before the element's content.
     */
    final public function block(): self
    {}

    /**
     * Makes an element eligible to work with flex-1 element's style.
     */
    final public function flex(): self
    {}

    /**
     * Makes an element grow and shrink as needed, ignoring the initial size.
     */
    final public function flex1(): self
    {}

    /**
     * Justifies childs along the element with an equal amount of space between.
     */
    final public function justifyBetween(): self
    {}

    /**
     * Justifies childs along the element with an equal amount of space between
     * each item and half around.
     */
    final public function justifyAround(): self
    {}

    /**
     * Justifies childs along the element with an equal amount of space around each item.
     */
    final public function justifyEvenly(): self
    {}

    /**
     * Justifies childs along the center of the container’s main axis.
     */
    final public function justifyCenter(): self
    {}

    /**
     * Repeats the string given until it fills all the content.
     */
    final public function contentRepeat(string $string): self
    {}

    /**
     * Prepends text to the content.
     */
    final public function prepend(string $string): self
    {}

    /**
     * Appends text to the content.
     */
    final public function append(string $string): self
    {}

    /**
     * Prepends the list style type to the content.
     */
    final public function list(string $type, int $index = 0): self
    {}

    /**
     * Adds the given properties to the element.
     *
     * @param  array<string, mixed>  $properties
     */
    public function with(array $properties): self
    {}

    /**
     * Sets the href property to the element.
     */
    final public function href(string $href): self
    {}

    /**
     * Formats a given string.
     */
    final public function format(string $content): string
    {}

    /**
     * Get the format string including required styles.
     */
    private function getFormatString(): string
    {}

    /**
     * Get the margins applied to the element.
     *
     * @return array{0: int, 1: int, 2: int, 3: int}
     */
    private function getMargins(): array
    {}

    /**
     * Get the paddings applied to the element.
     *
     * @return array{0: int, 1: int, 2: int, 3: int}
     */
    private function getPaddings(): array
    {}

    /**
     * It applies the correct width for the content.
     */
    private function applyWidth(string $content): string
    {}

    /**
     * It applies the styling for the content.
     */
    private function applyStyling(string $content): string
    {}

    /**
     * Get the length of the text provided without the styling tags.
     */
    public function getLength(?string $text = null): int
    {}

    /**
     * Get the length of the element without margins.
     */
    public function getInnerWidth(): int
    {}

    /**
     * Get the constant variant color from Color class.
     */
    private function getColorVariant(string $color, int $variant): string
    {}

    /**
     * Calculates the width based on the fraction provided.
     *
     * @param  array<string, int>  $styles
     * @param  array<string, array<int, int|string>>  $parentStyles
     */
    private static function calcWidthFromFraction(string $fraction, array $styles, array $parentStyles): int
    {}

    /**
     * Gets the width of the parent element.
     *
     * @param  array<string, array<int|string>>  $styles
     */
    public static function getParentWidth(array $styles): int
    {}

    /**
     * It trims the text properly ignoring all escape codes and
     * `<bg;fg;options>` tags.
     */
    private static function trimText(string $text, int $width): string
    {}
}
