<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * Additional emphasis processing code based on commonmark-java (https://github.com/atlassian/commonmark-java)
 *  - (c) Atlassian Pty Ltd
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Node\Inline;

use League\CommonMark\Node\Node;

/**
 * @internal
 */
final class AdjacentTextMerger
{
    public static function mergeChildNodes(Node $node): void
    {}

    public static function mergeTextNodesBetweenExclusive(Node $fromNode, Node $toNode): void
    {}

    public static function mergeWithDirectlyAdjacentNodes(Text $node): void
    {}

    private static function mergeTextNodesInclusive(Node $fromNode, Node $toNode): void
    {}

    private static function mergeIfNeeded(?Text $first, ?Text $last): void
    {}
}
