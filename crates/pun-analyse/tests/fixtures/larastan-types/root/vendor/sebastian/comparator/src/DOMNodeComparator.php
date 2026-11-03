<?php declare(strict_types=1);
/*
 * This file is part of sebastian/comparator.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Comparator;

use function assert;
use function mb_strtolower;
use function sprintf;
use DOMDocument;
use DOMNode;
use ValueError;

/**
 * An object exporter is not consulted for the representation of the DOMNode
 * objects that are compared: the representation this comparator provides for
 * them always has precedence.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for sebastian/comparator
 *
 * @internal This class is not covered by the backward compatibility promise for sebastian/comparator
 */
final class DOMNodeComparator extends ObjectComparator
{
    public function accepts(mixed $expected, mixed $actual): bool
    {}

    /**
     * @param array<mixed> $processed
     *
     * @throws ComparisonFailure
     */
    public function assertEquals(mixed $expected, mixed $actual, float $delta = 0.0, bool $canonicalize = false, bool $ignoreCase = false, array &$processed = []): void
    {}

    /**
     * Canonicalizes nodes, removes empty text nodes and merges adjacent text nodes,
     * and optionally ignores case.
     *
     * @see https://github.com/sebastianbergmann/phpunit/pull/1236#issuecomment-41765023
     */
    private function nodeToText(DOMNode $node, bool $ignoreCase): string
    {}

    /**
     * Serializes a node without canonicalization.
     * Used as a fallback when DOMNode::C14N() fails.
     */
    private function serialize(DOMNode $node): string
    {}
}
