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
use Symfony\Component\CssSelector\Exception\SyntaxErrorException;
use Symfony\Component\CssSelector\Node\FunctionNode;
use Symfony\Component\CssSelector\Parser\Parser;
use Symfony\Component\CssSelector\XPath\Translator;
use Symfony\Component\CssSelector\XPath\XPathExpr;

/**
 * XPath expression translator function extension.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class FunctionExtension extends AbstractExtension
{
    public function getFunctionTranslators(): array
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function translateNthChild(XPathExpr $xpath, FunctionNode $function, bool $last = false, bool $addNameTest = true): XPathExpr
    {}

    public function translateNthLastChild(XPathExpr $xpath, FunctionNode $function): XPathExpr
    {}

    public function translateNthOfType(XPathExpr $xpath, FunctionNode $function): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function translateNthLastOfType(XPathExpr $xpath, FunctionNode $function): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function translateContains(XPathExpr $xpath, FunctionNode $function): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function translateLang(XPathExpr $xpath, FunctionNode $function): XPathExpr
    {}

    public function getName(): string
    {}
}
