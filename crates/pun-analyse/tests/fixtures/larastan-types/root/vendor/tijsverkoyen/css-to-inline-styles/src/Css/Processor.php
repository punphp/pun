<?php

namespace TijsVerkoyen\CssToInlineStyles\Css;

use TijsVerkoyen\CssToInlineStyles\Css\Rule\Processor as RuleProcessor;
use TijsVerkoyen\CssToInlineStyles\Css\Rule\Rule;

class Processor
{
    /**
     * Get the rules from a given CSS-string
     *
     * @param string $css
     * @param Rule[] $existingRules
     *
     * @return Rule[]
     */
    public function getRules($css, $existingRules = array())
    {}

    /**
     * Get the CSS from the style-tags in the given HTML-string
     *
     * @param string $html
     *
     * @return string
     */
    public function getCssFromStyleTags($html)
    {}

    /**
     * @param string $css
     *
     * @return string
     */
    private function doCleanup($css)
    {}
}
