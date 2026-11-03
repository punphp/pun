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
use Symfony\Component\CssSelector\XPath\XPathExpr;

/**
 * XPath expression translator pseudo-class extension.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class PseudoClassExtension extends AbstractExtension
{
    public function getPseudoClassTranslators(): array
    {}

    public function translateRoot(XPathExpr $xpath): XPathExpr
    {}

    public function translateScopePseudo(XPathExpr $xpath): XPathExpr
    {}

    public function translateFirstChild(XPathExpr $xpath): XPathExpr
    {}

    public function translateLastChild(XPathExpr $xpath): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function translateFirstOfType(XPathExpr $xpath): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function translateLastOfType(XPathExpr $xpath): XPathExpr
    {}

    public function translateOnlyChild(XPathExpr $xpath): XPathExpr
    {}

    public function translateOnlyOfType(XPathExpr $xpath): XPathExpr
    {}

    public function translateEmpty(XPathExpr $xpath): XPathExpr
    {}

    public function getName(): string
    {}
}
