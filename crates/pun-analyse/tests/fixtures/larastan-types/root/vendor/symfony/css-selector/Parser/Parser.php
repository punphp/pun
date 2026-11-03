<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\Parser;

use Symfony\Component\CssSelector\Exception\InternalErrorException;
use Symfony\Component\CssSelector\Exception\SyntaxErrorException;
use Symfony\Component\CssSelector\Node;
use Symfony\Component\CssSelector\Parser\Tokenizer\Tokenizer;

/**
 * CSS selector parser.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/scrapy/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class Parser implements ParserInterface
{
    private const HAS_NESTING_LIMIT = 16;
    private const NESTING_LIMIT = 16;

    private Tokenizer $tokenizer;
    private int $hasNestingDepth = 0;
    private int $nestingDepth = 0;

    public function __construct(?Tokenizer $tokenizer = null)
    {}

    public function parse(string $source): array
    {}

    /**
     * Parses the arguments for ":nth-child()" and friends.
     *
     * @param Token[] $tokens
     *
     * @throws SyntaxErrorException
     */
    public static function parseSeries(array $tokens): array
    {}

    private function parseSelectorList(TokenStream $stream, bool $isArgument = false): array
    {}

    private function parserSelectorNode(TokenStream $stream, bool $isArgument = false, bool $insideRelativeSelector = false): Node\SelectorNode
    {}

    /**
     * @return list<array{0: string, 1: Node\SelectorNode}>
     *
     * @throws SyntaxErrorException
     * @throws InternalErrorException
     */
    private function parseRelativeSelector(TokenStream $stream): array
    {}

    /**
     * Parses next simple node (hash, class, pseudo, negation).
     *
     * @throws SyntaxErrorException
     * @throws InternalErrorException
     */
    private function parseSimpleSelector(TokenStream $stream, bool $insideNegation = false, bool $isArgument = false, bool $insideRelativeSelector = false): array
    {}

    /**
     * @return Node\SelectorNode[]
     *
     * @throws SyntaxErrorException
     */
    private function parseNestedSelectorList(TokenStream $stream, string $identifier): array
    {}

    private function parseElementNode(TokenStream $stream): Node\ElementNode
    {}

    private function parseAttributeNode(Node\NodeInterface $selector, TokenStream $stream): Node\AttributeNode
    {}
}
