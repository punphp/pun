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

use Symfony\Component\CssSelector\XPath\XPathExpr;

/**
 * XPath expression translator combination extension.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class CombinationExtension extends AbstractExtension
{
    public function getCombinationTranslators(): array
    {}

    public function translateDescendant(XPathExpr $xpath, XPathExpr $combinedXpath): XPathExpr
    {}

    public function translateChild(XPathExpr $xpath, XPathExpr $combinedXpath): XPathExpr
    {}

    public function translateDirectAdjacent(XPathExpr $xpath, XPathExpr $combinedXpath): XPathExpr
    {}

    public function translateIndirectAdjacent(XPathExpr $xpath, XPathExpr $combinedXpath): XPathExpr
    {}

    public function getName(): string
    {}

    /**
     * Replaces position() and last() by tests on the siblings.
     *
     * On the following-sibling axis, position() and last() don't count the preceding siblings.
     */
    private function positionsToSiblings(XPathExpr $xpath): XPathExpr
    {}
}
