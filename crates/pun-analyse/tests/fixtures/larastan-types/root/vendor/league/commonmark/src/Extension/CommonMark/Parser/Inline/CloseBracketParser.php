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

namespace League\CommonMark\Extension\CommonMark\Parser\Inline;

use League\CommonMark\Delimiter\Bracket;
use League\CommonMark\Environment\EnvironmentAwareInterface;
use League\CommonMark\Environment\EnvironmentInterface;
use League\CommonMark\Extension\CommonMark\Node\Inline\AbstractWebResource;
use League\CommonMark\Extension\CommonMark\Node\Inline\Image;
use League\CommonMark\Extension\CommonMark\Node\Inline\Link;
use League\CommonMark\Extension\Mention\Mention;
use League\CommonMark\Node\Inline\AdjacentTextMerger;
use League\CommonMark\Node\Inline\Text;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Parser\Inline\InlineParserInterface;
use League\CommonMark\Parser\Inline\InlineParserMatch;
use League\CommonMark\Parser\InlineParserContext;
use League\CommonMark\Reference\ReferenceInterface;
use League\CommonMark\Reference\ReferenceMapInterface;
use League\CommonMark\Util\LinkParserHelper;
use League\CommonMark\Util\RegexHelper;

final class CloseBracketParser implements InlineParserInterface, EnvironmentAwareInterface
{
    /** @psalm-readonly-allow-private-mutation */
    private EnvironmentInterface $environment;

    public function getMatchDefinition(): InlineParserMatch
    {}

    public function parse(InlineParserContext $inlineContext): bool
    {}

    public function setEnvironment(EnvironmentInterface $environment): void
    {}

    /**
     * @return array<string, string>|null
     */
    private function tryParseInlineLinkAndTitle(Cursor $cursor): ?array
    {}

    private function tryParseReference(Cursor $cursor, ReferenceMapInterface $referenceMap, Bracket $opener, int $startPos): ?ReferenceInterface
    {}

    private function createInline(string $url, string $title, bool $isImage, ?ReferenceInterface $reference = null): AbstractWebResource
    {}
}
