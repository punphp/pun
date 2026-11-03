<?php

namespace TijsVerkoyen\CssToInlineStyles;

use Symfony\Component\CssSelector\CssSelectorConverter;
use Symfony\Component\CssSelector\Exception\ExceptionInterface;
use TijsVerkoyen\CssToInlineStyles\Css\Processor;
use TijsVerkoyen\CssToInlineStyles\Css\Property\Processor as PropertyProcessor;
use TijsVerkoyen\CssToInlineStyles\Css\Property\Property;
use TijsVerkoyen\CssToInlineStyles\Css\Rule\Processor as RuleProcessor;

class CssToInlineStyles
{
    /**
     * @var CssSelectorConverter
     */
    private $cssConverter;

    public function __construct()
    {}

    /**
     * Will inline the $css into the given $html
     *
     * Remark: if the html contains <style>-tags those will be used, the rules
     * in $css will be appended.
     *
     * @param string $html
     * @param string $css
     *
     * @return string
     */
    public function convert($html, $css = null)
    {}

    /**
     * Inline the given properties on a given DOMElement
     *
     * @param \DOMElement             $element
     * @param Property[] $properties
     *
     * @return \DOMElement
     */
    public function inlineCssOnElement(\DOMElement $element, array $properties)
    {}

    /**
     * Get the current inline styles for a given DOMElement
     *
     * @param \DOMElement $element
     *
     * @return Property[]
     */
    public function getInlineStyles(\DOMElement $element)
    {}

    /**
     * @param string $html
     *
     * @return \DOMDocument
     */
    protected function createDomDocumentFromHtml($html)
    {}

    /**
     * @param \DOMDocument $document
     *
     * @return string
     */
    protected function getHtmlFromDocument(\DOMDocument $document)
    {}

    /**
     * @param \DOMDocument    $document
     * @param Css\Rule\Rule[] $rules
     *
     * @return \DOMDocument
     */
    protected function inline(\DOMDocument $document, array $rules)
    {}

    /**
     * Merge the CSS rules to determine the applied properties.
     *
     * @param Property[] $properties
     * @param array<string, Property> $cssProperties existing applied properties indexed by name
     *
     * @return array<string, Property> updated properties, indexed by name
     */
    private function calculatePropertiesToBeApplied(array $properties, array $cssProperties): array
    {}
}
