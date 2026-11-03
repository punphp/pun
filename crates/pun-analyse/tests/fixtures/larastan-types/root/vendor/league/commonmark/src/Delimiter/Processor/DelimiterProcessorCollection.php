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
 * Additional emphasis processing code based on commonmark-java (https://github.com/atlassian/commonmark-java)
 *  - (c) Atlassian Pty Ltd
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Delimiter\Processor;

use League\CommonMark\Exception\InvalidArgumentException;

final class DelimiterProcessorCollection implements DelimiterProcessorCollectionInterface
{
    /**
     * @var array<string,DelimiterProcessorInterface>|DelimiterProcessorInterface[]
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $processorsByChar = [];

    public function add(DelimiterProcessorInterface $processor): void
    {}

    public function getDelimiterProcessor(string $char): ?DelimiterProcessorInterface
    {}

    /**
     * @return string[]
     */
    public function getDelimiterCharacters(): array
    {}

    private function addDelimiterProcessorForChar(string $delimiterChar, DelimiterProcessorInterface $processor): void
    {}

    private function addStaggeredDelimiterProcessorForChar(string $opening, DelimiterProcessorInterface $old, DelimiterProcessorInterface $new): void
    {}

    public function count(): int
    {}
}
