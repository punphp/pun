<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * Original code based on the CommonMark JS reference parser (https://bitly.com/commonmark-js)
 *  - (c) John MacFarlane
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Node\Block;

use League\CommonMark\Exception\InvalidArgumentException;
use League\CommonMark\Node\Node;

/**
 * Block-level element
 *
 * @method parent() ?AbstractBlock
 */
abstract class AbstractBlock extends Node
{
    protected ?int $startLine = null;

    protected ?int $endLine = null;

    protected function setParent(?Node $node = null): void
    {}

    public function setStartLine(?int $startLine): void
    {}

    public function getStartLine(): ?int
    {}

    public function setEndLine(?int $endLine): void
    {}

    public function getEndLine(): ?int
    {}
}
