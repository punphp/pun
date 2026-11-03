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

use League\CommonMark\Parser\Cursor;
use League\CommonMark\Reference\ReferenceMap;
use League\CommonMark\Reference\ReferenceMapInterface;

class Document extends AbstractBlock
{
    /** @psalm-readonly */
    protected ReferenceMapInterface $referenceMap;

    public function __construct(?ReferenceMapInterface $referenceMap = null)
    {}

    public function getReferenceMap(): ReferenceMapInterface
    {}

    public function canContain(AbstractBlock $block): bool
    {}

    public function isCode(): bool
    {}

    public function matchesNextLine(Cursor $cursor): bool
    {}
}
