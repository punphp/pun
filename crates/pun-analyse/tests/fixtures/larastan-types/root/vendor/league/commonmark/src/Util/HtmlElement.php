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

namespace League\CommonMark\Util;

final class HtmlElement implements \Stringable
{
    /** @psalm-readonly */
    private string $tagName;

    /** @var array<string, string|bool> */
    private array $attributes = [];

    /** @var \Stringable|\Stringable[]|string */
    private $contents;

    /** @psalm-readonly */
    private bool $selfClosing;

    /**
     * @param string                                $tagName     Name of the HTML tag
     * @param array<string, string|string[]|bool>   $attributes  Array of attributes (values should be unescaped)
     * @param \Stringable|\Stringable[]|string|null $contents    Inner contents, pre-escaped if needed
     * @param bool                                  $selfClosing Whether the tag is self-closing
     */
    public function __construct(string $tagName, array $attributes = [], $contents = '', bool $selfClosing = false)
    {}

    /** @psalm-immutable */
    public function getTagName(): string
    {}

    /**
     * @return array<string, string|bool>
     *
     * @psalm-immutable
     */
    public function getAllAttributes(): array
    {}

    /**
     * @return string|bool|null
     *
     * @psalm-immutable
     */
    public function getAttribute(string $key)
    {}

    /**
     * @param string|string[]|bool $value
     */
    public function setAttribute(string $key, $value = true): self
    {}

    /**
     * @return \Stringable|\Stringable[]|string
     *
     * @psalm-immutable
     */
    public function getContents(bool $asString = true)
    {}

    /**
     * Sets the inner contents of the tag (must be pre-escaped if needed)
     *
     * @param \Stringable|\Stringable[]|string $contents
     *
     * @return $this
     */
    public function setContents($contents): self
    {}

    /** @psalm-immutable */
    public function __toString(): string
    {}

    /** @psalm-immutable */
    private function getContentsAsString(): string
    {}
}
