<?php

declare(strict_types=1);

namespace Termwind;

use DOMDocument;
use DOMNode;
use Termwind\Html\CodeRenderer;
use Termwind\Html\PreRenderer;
use Termwind\Html\TableRenderer;
use Termwind\ValueObjects\Node;

/**
 * @internal
 */
final class HtmlRenderer
{
    /**
     * Renders the given html.
     *
     * @param  int<0, 511>  $options
     */
    public function render(string $html, int $options): void
    {}

    /**
     * Parses the given html.
     */
    public function parse(string $html): Components\Element
    {}

    /**
     * Convert a tree of DOM nodes to a tree of termwind elements.
     */
    private function convert(Node $node): Components\Element|string
    {}

    /**
     * Convert a given DOM node to it's termwind element equivalent.
     *
     * @param  array<int, Components\Element|string>  $children
     */
    private function toElement(Node $node, array $children): Components\Element|string
    {}
}
