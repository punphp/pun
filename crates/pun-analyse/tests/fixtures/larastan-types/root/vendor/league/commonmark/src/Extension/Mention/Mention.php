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

namespace League\CommonMark\Extension\Mention;

use League\CommonMark\Extension\CommonMark\Node\Inline\Link;
use League\CommonMark\Node\Inline\Text;

class Mention extends Link
{
    private string $name;

    private string $prefix;

    private string $identifier;

    public function __construct(string $name, string $prefix, string $identifier, ?string $label = null)
    {}

    public function getLabel(): ?string
    {}

    public function getIdentifier(): string
    {}

    public function getName(): ?string
    {}

    public function getPrefix(): string
    {}

    public function hasUrl(): bool
    {}

    /**
     * @return $this
     */
    public function setLabel(string $label): self
    {}

    private function findLabelNode(): ?Text
    {}
}
