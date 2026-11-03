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
use Symfony\Component\CssSelector\Node\FunctionNode;
use Symfony\Component\CssSelector\XPath\Translator;
use Symfony\Component\CssSelector\XPath\XPathExpr;

/**
 * XPath expression translator HTML extension.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class HtmlExtension extends AbstractExtension
{
    // Each disabled fieldset ancestor disables the element, except the one whose first legend
    // child the element sits in. Those excepted fieldsets are exactly the parents of the
    // ancestors-or-self that are a first legend child of a disabled fieldset, one for one, so
    // the element is disabled as soon as the first count exceeds the second one. Counting keeps
    // this to two cheap ancestor walks instead of a predicate run on every ancestor.
    private const DISABLING_FIELDSET = 'count(ancestor::fieldset[@disabled]) > count(ancestor-or-self::legend[not(preceding-sibling::legend)][parent::fieldset[@disabled]])';

    public function __construct(Translator $translator)
    {}

    public function getPseudoClassTranslators(): array
    {}

    public function getFunctionTranslators(): array
    {}

    public function translateChecked(XPathExpr $xpath): XPathExpr
    {}

    public function translateLink(XPathExpr $xpath): XPathExpr
    {}

    public function translateDisabled(XPathExpr $xpath): XPathExpr
    {}

    public function translateEnabled(XPathExpr $xpath): XPathExpr
    {}

    /**
     * @throws ExpressionErrorException
     */
    public function translateLang(XPathExpr $xpath, FunctionNode $function): XPathExpr
    {}

    public function translateSelected(XPathExpr $xpath): XPathExpr
    {}

    public function translateInvalid(XPathExpr $xpath): XPathExpr
    {}

    public function translateHover(XPathExpr $xpath): XPathExpr
    {}

    public function translateVisited(XPathExpr $xpath): XPathExpr
    {}

    public function getName(): string
    {}
}
