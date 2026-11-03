<?php

namespace Illuminate\Validation;

use Illuminate\Support\Arr;

class ValidationData
{
    /**
     * Initialize and gather data for the given attribute.
     *
     * @param  string  $attribute
     * @param  array  $masterData
     * @return array
     */
    public static function initializeAndGatherData($attribute, $masterData)
    {}

    /**
     * Gather a copy of the attribute data filled with any missing attributes.
     *
     * @param  string  $attribute
     * @param  array  $masterData
     * @return array
     */
    protected static function initializeAttributeOnData($attribute, $masterData)
    {}

    /**
     * Get all of the exact attribute values for a given wildcard attribute.
     *
     * @param  array  $masterData
     * @param  array  $data
     * @param  string  $attribute
     * @return array
     */
    protected static function extractValuesForWildcards($masterData, $data, $attribute)
    {}

    /**
     * Extract data based on the given dot-notated path.
     *
     * Used to extract a sub-section of the data for faster iteration.
     *
     * @param  string  $attribute
     * @param  array  $masterData
     * @return array
     */
    public static function extractDataFromPath($attribute, $masterData)
    {}

    /**
     * Get the explicit part of the attribute name.
     *
     * E.g. 'foo.bar.*.baz' -> 'foo.bar'
     *
     * Allows us to not spin through all of the flattened data for some operations.
     *
     * @param  string  $attribute
     * @return string
     */
    public static function getLeadingExplicitAttributePath($attribute)
    {}
}
