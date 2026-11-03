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

namespace League\CommonMark\Extension\CommonMark\Node\Block;

use League\CommonMark\Node\Block\AbstractBlock;
use League\CommonMark\Node\Block\TightBlockInterface;

class ListBlock extends AbstractBlock implements TightBlockInterface
{
    public const TYPE_BULLET  = 'bullet';
    public const TYPE_ORDERED = 'ordered';

    public const DELIM_PERIOD = 'period';
    public const DELIM_PAREN  = 'paren';

    protected bool $tight = false; // TODO Make lists tight by default in v3

    /** @psalm-readonly */
    protected ListData $listData;

    public function __construct(ListData $listData)
    {}

    public function getListData(): ListData
    {}

    public function isTight(): bool
    {}

    public function setTight(bool $tight): void
    {}
}
