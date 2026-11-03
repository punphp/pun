<?php

namespace TijsVerkoyen\CssToInlineStyles\Css\Rule;

use Symfony\Component\CssSelector\Node\Specificity;
use \TijsVerkoyen\CssToInlineStyles\Css\Property\Processor as PropertyProcessor;

class Processor
{
    /**
     * Splits a string into separate rules
     *
     * @param string $rulesString
     *
     * @return string[]
     */
    public function splitIntoSeparateRules($rulesString)
    {}

    /**
     * @param string $string
     *
     * @return string
     */
    private function cleanup($string)
    {}

    /**
     * Converts a rule-string into an object
     *
     * @param string $rule
     * @param int    $originalOrder
     *
     * @return Rule[]
     */
    public function convertToObjects($rule, $originalOrder)
    {}

    /**
     * Calculates the specificity based on a CSS Selector string,
     * Based on the patterns from premailer/css_parser by Alex Dunae
     *
     * @see https://github.com/premailer/css_parser/blob/master/lib/css_parser/regexps.rb
     *
     * @param string $selector
     *
     * @return Specificity
     */
    public function calculateSpecificityBasedOnASelector($selector)
    {}

    /**
     * @param string[] $rules
     * @param Rule[]   $objects
     *
     * @return Rule[]
     */
    public function convertArrayToObjects(array $rules, array $objects = array())
    {}

    /**
     * Sorts an array on the specificity element in an ascending way
     * Lower specificity will be sorted to the beginning of the array
     *
     * @param Rule $e1 The first element.
     * @param Rule $e2 The second element.
     *
     * @return int
     */
    public static function sortOnSpecificity(Rule $e1, Rule $e2)
    {}
}
