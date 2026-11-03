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

use Symfony\Component\CssSelector\XPath\Translator;
use Symfony\Component\CssSelector\XPath\XPathExpr;

/**
 * XPath expression translator attribute extension.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class AttributeMatchingExtension extends AbstractExtension
{
    public function getAttributeMatchingTranslators(): array
    {}

    public function translateExists(XPathExpr $xpath, string $attribute, ?string $value): XPathExpr
    {}

    public function translateEquals(XPathExpr $xpath, string $attribute, ?string $value): XPathExpr
    {}

    public function translateIncludes(XPathExpr $xpath, string $attribute, ?string $value): XPathExpr
    {}

    public function translateDashMatch(XPathExpr $xpath, string $attribute, ?string $value): XPathExpr
    {}

    public function translatePrefixMatch(XPathExpr $xpath, string $attribute, ?string $value): XPathExpr
    {}

    public function translateSuffixMatch(XPathExpr $xpath, string $attribute, ?string $value): XPathExpr
    {}

    public function translateSubstringMatch(XPathExpr $xpath, string $attribute, ?string $value): XPathExpr
    {}

    public function translateDifferent(XPathExpr $xpath, string $attribute, ?string $value): XPathExpr
    {}

    public function getName(): string
    {}
}
