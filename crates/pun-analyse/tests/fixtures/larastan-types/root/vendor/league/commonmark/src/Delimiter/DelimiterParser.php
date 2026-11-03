<?php

declare(strict_types=1);

namespace League\CommonMark\Delimiter;

use League\CommonMark\Delimiter\Processor\DelimiterProcessorCollection;
use League\CommonMark\Delimiter\Processor\DelimiterProcessorInterface;
use League\CommonMark\Node\Inline\Text;
use League\CommonMark\Parser\Inline\InlineParserInterface;
use League\CommonMark\Parser\Inline\InlineParserMatch;
use League\CommonMark\Parser\InlineParserContext;
use League\CommonMark\Util\RegexHelper;

/**
 * Delimiter parsing is implemented as an Inline Parser with the lowest-possible priority
 *
 * @internal
 */
final class DelimiterParser implements InlineParserInterface
{
    private DelimiterProcessorCollection $collection;

    public function __construct(DelimiterProcessorCollection $collection)
    {}

    public function getMatchDefinition(): InlineParserMatch
    {}

    public function parse(InlineParserContext $inlineContext): bool
    {}

    /**
     * @return bool[]
     */
    private static function determineCanOpenOrClose(string $charBefore, string $charAfter, string $character, DelimiterProcessorInterface $delimiterProcessor): array
    {}
}
