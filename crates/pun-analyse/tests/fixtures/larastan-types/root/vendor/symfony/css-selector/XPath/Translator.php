<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\XPath;

use Symfony\Component\CssSelector\Exception\ExpressionErrorException;
use Symfony\Component\CssSelector\Node\CombinedSelectorNode;
use Symfony\Component\CssSelector\Node\FunctionNode;
use Symfony\Component\CssSelector\Node\NodeInterface;
use Symfony\Component\CssSelector\Node\SelectorNode;
use Symfony\Component\CssSelector\Parser\Parser;
use Symfony\Component\CssSelector\Parser\ParserInterface;

/**
 * XPath expression translator interface.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class Translator implements TranslatorInterface
{
    private ParserInterface $mainParser;

    /**
     * @var ParserInterface[]
     */
    private array $shortcutParsers = [];

    /**
     * @var Extension\ExtensionInterface[]
     */
    private array $extensions = [];

    private array $nodeTranslators = [];
    private array $combinationTranslators = [];
    private array $relativeCombinationTranslators = [];
    private array $functionTranslators = [];
    private array $pseudoClassTranslators = [];
    private array $attributeMatchingTranslators = [];

    public function __construct(?ParserInterface $parser = null)
    {}

    public static function getXpathLiteral(string $element): string
    {}

    public function cssToXPath(string $cssExpr, string $prefix = 'descendant-or-self::'): string
    {}

    public function selectorToXPath(SelectorNode $selector, string $prefix = 'descendant-or-self::'): string
    {}

    /**
     * @return $this
     */
    public function registerExtension(Extension\ExtensionInterface $extension): static
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function getExtension(string $name): Extension\ExtensionInterface
    {}

    /**
     * @return $this
     */
    public function registerParserShortcut(ParserInterface $shortcut): static
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function nodeToXPath(NodeInterface $node): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function addCombination(string $combiner, NodeInterface $xpath, NodeInterface $combinedXpath): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function addRelativeCombination(string $combiner, NodeInterface $xpath, NodeInterface $combinedXpath): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function addFunction(XPathExpr $xpath, FunctionNode $function): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function addPseudoClass(XPathExpr $xpath, string $pseudoClass): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function addAttributeMatching(XPathExpr $xpath, string $operator, string $attribute, ?string $value): XPathExpr
    {}

    /**
     * Returns the path from the element that :has() applies to, to the subject of the relative selector.
     *
     * @throws ExpressionErrorException
     */
    private function relativeToXPath(string $combiner, NodeInterface $node): XPathExpr
    {}

    /**
     * @return SelectorNode[]
     */
    private function parseSelectors(string $css): array
    {}
}
