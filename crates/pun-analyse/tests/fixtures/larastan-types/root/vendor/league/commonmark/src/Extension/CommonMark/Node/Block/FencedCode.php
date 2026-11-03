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
use League\CommonMark\Node\StringContainerInterface;

final class FencedCode extends AbstractBlock implements StringContainerInterface
{
    private ?string $info = null;

    private string $literal = '';

    private int $length;

    private string $char;

    private int $offset;

    public function __construct(int $length, string $char, int $offset)
    {}

    public function getInfo(): ?string
    {}

    /**
     * @return string[]
     */
    public function getInfoWords(): array
    {}

    public function setInfo(string $info): void
    {}

    public function getLiteral(): string
    {}

    public function setLiteral(string $literal): void
    {}

    public function getChar(): string
    {}

    public function setChar(string $char): void
    {}

    public function getLength(): int
    {}

    public function setLength(int $length): void
    {}

    public function getOffset(): int
    {}

    public function setOffset(int $offset): void
    {}
}
