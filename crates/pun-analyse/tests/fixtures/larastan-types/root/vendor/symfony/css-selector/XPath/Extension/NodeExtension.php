<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\XPath\Extension;

use Symfony\Component\CssSelector\Exception\ExpressionErrorException;
use Symfony\Component\CssSelector\Node;
use Symfony\Component\CssSelector\XPath\Translator;
use Symfony\Component\CssSelector\XPath\XPathExpr;

/**
 * XPath expression translator node extension.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/scrapy/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class NodeExtension extends AbstractExtension
{
    public const ELEMENT_NAME_IN_LOWER_CASE = 1;
    public const ATTRIBUTE_NAME_IN_LOWER_CASE = 2;
    public const ATTRIBUTE_VALUE_IN_LOWER_CASE = 4;

    public function __construct(
        private int $flags = 0,
    ) {}

    /**
     * @return $this
     */
    public function setFlag(int $flag, bool $on): static
    {}

    public function hasFlag(int $flag): bool
    {}

    public function getNodeTranslators(): array
    {}

    public function translateSelector(Node\SelectorNode $node, Translator $translator): XPathExpr
    {}

    public function translateCombinedSelector(Node\CombinedSelectorNode $node, Translator $translator): XPathExpr
    {}

    public function translateNegation(Node\NegationNode $node, Translator $translator): XPathExpr
    {}

    public function translateMatching(Node\MatchingNode $node, Translator $translator): XPathExpr
    {}

    public function translateSpecificityAdjustment(Node\SpecificityAdjustmentNode $node, Translator $translator): XPathExpr
    {}

    /**
     * @param array<Node\NodeInterface> $arguments
     */
    private function translateMatchingOrSpecificityAdjustment(Node\NodeInterface $selector, array $arguments, Translator $translator): XPathExpr
    {}

    public function translateFunction(Node\FunctionNode $node, Translator $translator): XPathExpr
    {}

    public function translatePseudo(Node\PseudoNode $node, Translator $translator): XPathExpr
    {}

    public function translateAttribute(Node\AttributeNode $node, Translator $translator): XPathExpr
    {}

    public function translateClass(Node\ClassNode $node, Translator $translator): XPathExpr
    {}

    public function translateHash(Node\HashNode $node, Translator $translator): XPathExpr
    {}

    public function translateElement(Node\ElementNode $node): XPathExpr
    {}

    public function translateRelation(Node\RelationNode $node, Translator $translator): XPathExpr
    {}

    public function getName(): string
    {}

    private function hasNthOfType(Node\NodeInterface $node): bool
    {}

    private function getElementType(Node\NodeInterface $node): string
    {}

    private function hasUntypedOfType(Node\NodeInterface $node): bool
    {}

    private function getElementNode(Node\NodeInterface $node): ?Node\ElementNode
    {}

    private function withElement(Node\NodeInterface $node, Node\ElementNode $element): Node\NodeInterface
    {}

    private function isSafeName(string $name): bool
    {}

    /**
     * Returns the condition for the context node to match the selector, using reverse axes for its combinators.
     */
    private function selectorCondition(Node\NodeInterface $node, Translator $translator, string $type = '*'): string
    {}
}
