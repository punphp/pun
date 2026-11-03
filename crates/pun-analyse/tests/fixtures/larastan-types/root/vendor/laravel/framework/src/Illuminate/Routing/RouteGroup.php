<?php

namespace Illuminate\Routing;

use Illuminate\Support\Arr;

class RouteGroup
{
    /**
     * Merge route groups into a new array.
     *
     * @param  array  $new
     * @param  array  $old
     * @param  bool  $prependExistingPrefix
     * @return array
     */
    public static function merge($new, $old, $prependExistingPrefix = true)
    {}

    /**
     * Format the metadata for the new group attributes.
     *
     * @param  array  $new
     * @param  array  $old
     * @return array
     */
    protected static function formatMetadata($new, $old)
    {}

    /**
     * Merge the given route metadata.
     *
     * Associative array values are merged recursively, while all other values, including lists, replace the existing value entirely.
     *
     * @param  array  $old
     * @param  array  $new
     * @return array
     */
    public static function mergeMetadata(array $old, array $new)
    {}

    /**
     * Determine if the given metadata values should be merged.
     *
     * @param  mixed  $old
     * @param  mixed  $new
     * @return bool
     */
    protected static function mergableMetadata($old, $new)
    {}

    /**
     * Format the namespace for the new group attributes.
     *
     * @param  array  $new
     * @param  array  $old
     * @return string|null
     */
    protected static function formatNamespace($new, $old)
    {}

    /**
     * Format the prefix for the new group attributes.
     *
     * @param  array  $new
     * @param  array  $old
     * @param  bool  $prependExistingPrefix
     * @return string|null
     */
    protected static function formatPrefix($new, $old, $prependExistingPrefix = true)
    {}

    /**
     * Format the "wheres" for the new group attributes.
     *
     * @param  array  $new
     * @param  array  $old
     * @return array
     */
    protected static function formatWhere($new, $old)
    {}

    /**
     * Format the "as" clause of the new group attributes.
     *
     * @param  array  $new
     * @param  array  $old
     * @return array
     */
    protected static function formatAs($new, $old)
    {}
}
