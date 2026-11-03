<?php

declare(strict_types=1);

namespace Termwind\Html;

use Termwind\Components\Element;
use Termwind\Termwind;
use Termwind\ValueObjects\Styles;

/**
 * @internal
 */
final class InheritStyles
{
    /**
     * Applies styles from parent element to child elements.
     *
     * @param  array<int, Element|string>  $elements
     * @return array<int, Element|string>
     */
    public function __invoke(array $elements, Styles $styles): array
    {}

    /**
     * Applies flex-1 to child elements with the class.
     *
     * @param  array<int, Element>  $elements
     * @return array<int, Element>
     */
    private function applyFlex(array $elements): array
    {}

    /**
     * Applies the space between the elements.
     *
     * @param  array<int, Element>  $elements
     * @return array<int, Element|string>
     */
    private function applyJustifyBetween(array $elements): array
    {}

    /**
     * Applies the space between and around the elements.
     *
     * @param  array<int, Element>  $elements
     * @return array<int, Element|string>
     */
    private function applyJustifyEvenly(array $elements): array
    {}

    /**
     * Applies the space around the elements.
     *
     * @param  array<int, Element>  $elements
     * @return array<int, Element|string>
     */
    private function applyJustifyAround(array $elements): array
    {}

    /**
     * Applies the space on before first element and after last element.
     *
     * @param  array<int, Element>  $elements
     * @return array<int, Element|string>
     */
    private function applyJustifyCenter(array $elements): array
    {}

    /**
     * Gets the total width for the elements and their parent width.
     *
     * @param  array<int, Element>  $elements
     * @return int[]
     */
    private function getWidthFromElements(array $elements)
    {}
}
