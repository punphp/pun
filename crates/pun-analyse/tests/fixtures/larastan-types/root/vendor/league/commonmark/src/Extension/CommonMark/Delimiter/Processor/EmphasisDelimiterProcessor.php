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

namespace League\CommonMark\Extension\CommonMark\Delimiter\Processor;

use League\CommonMark\Delimiter\DelimiterInterface;
use League\CommonMark\Delimiter\Processor\CacheableDelimiterProcessorInterface;
use League\CommonMark\Extension\CommonMark\Node\Inline\Emphasis;
use League\CommonMark\Extension\CommonMark\Node\Inline\Strong;
use League\CommonMark\Node\Inline\AbstractStringContainer;
use League\Config\ConfigurationAwareInterface;
use League\Config\ConfigurationInterface;

final class EmphasisDelimiterProcessor implements CacheableDelimiterProcessorInterface, ConfigurationAwareInterface
{
    /** @psalm-readonly */
    private string $char;

    /** @psalm-readonly-allow-private-mutation */
    private ConfigurationInterface $config;

    /**
     * @param string $char The emphasis character to use (typically '*' or '_')
     */
    public function __construct(string $char)
    {}

    public function getOpeningCharacter(): string
    {}

    public function getClosingCharacter(): string
    {}

    public function getMinLength(): int
    {}

    public function getDelimiterUse(DelimiterInterface $opener, DelimiterInterface $closer): int
    {}

    public function process(AbstractStringContainer $opener, AbstractStringContainer $closer, int $delimiterUse): void
    {}

    public function setConfiguration(ConfigurationInterface $configuration): void
    {}

    public function getCacheKey(DelimiterInterface $closer): string
    {}
}
