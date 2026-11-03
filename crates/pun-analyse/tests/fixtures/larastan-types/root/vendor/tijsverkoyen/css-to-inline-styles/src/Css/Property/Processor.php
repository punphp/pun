<?php

namespace TijsVerkoyen\CssToInlineStyles\Css\Property;

use Symfony\Component\CssSelector\Node\Specificity;

class Processor
{
    /**
     * Split a string into separate properties
     *
     * @param string $propertiesString
     *
     * @return string[]
     */
    public function splitIntoSeparateProperties($propertiesString)
    {}

    /**
     * @param string $string
     *
     * @return string
     */
    private function cleanup($string)
    {}

    /**
     * Converts a property-string into an object
     *
     * @param string $property
     *
     * @return Property|null
     */
    public function convertToObject($property, ?Specificity $specificity = null)
    {}

    /**
     * Converts an array of property-strings into objects
     *
     * @param string[] $properties
     *
     * @return Property[]
     */
    public function convertArrayToObjects(array $properties, ?Specificity $specificity = null)
    {}

    /**
     * Build the property-string for multiple properties
     *
     * @param Property[] $properties
     *
     * @return string
     */
    public function buildPropertiesString(array $properties)
    {}
}
