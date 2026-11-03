<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Extension\CommonMark\Parser\Block;

use League\CommonMark\Extension\CommonMark\Node\Block\ListBlock;
use League\CommonMark\Extension\CommonMark\Node\Block\ListData;
use League\CommonMark\Parser\Block\BlockStart;
use League\CommonMark\Parser\Block\BlockStartParserInterface;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Parser\MarkdownParserStateInterface;
use League\CommonMark\Util\RegexHelper;
use League\Config\ConfigurationAwareInterface;
use League\Config\ConfigurationInterface;

final class ListBlockStartParser implements BlockStartParserInterface, ConfigurationAwareInterface
{
    /** @psalm-readonly-allow-private-mutation */
    private ?ConfigurationInterface $config = null;

    /**
     * @psalm-var non-empty-string|null
     *
     * @psalm-readonly-allow-private-mutation
     */
    private ?string $listMarkerRegex = null;

    public function setConfiguration(ConfigurationInterface $configuration): void
    {}

    public function tryStart(Cursor $cursor, MarkdownParserStateInterface $parserState): ?BlockStart
    {}

    private function parseList(Cursor $cursor, bool $inParagraph): ?ListData
    {}

    private static function calculateListMarkerPadding(Cursor $cursor, int $markerLength): int
    {}

    /**
     * @psalm-return non-empty-string
     */
    private function generateListMarkerRegex(): string
    {}
}
