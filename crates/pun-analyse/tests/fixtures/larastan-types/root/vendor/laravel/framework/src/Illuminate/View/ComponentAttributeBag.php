<?php

namespace Illuminate\View;

use ArrayAccess;
use ArrayIterator;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\HtmlString;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\InteractsWithData;
use Illuminate\Support\Traits\Macroable;
use IteratorAggregate;
use JsonSerializable;
use Stringable;
use Traversable;

class ComponentAttributeBag implements Arrayable, ArrayAccess, IteratorAggregate, JsonSerializable, Htmlable, Stringable
{
    use Conditionable, InteractsWithData, Macroable;

    /**
     * The raw array of attributes.
     *
     * @var array
     */
    protected $attributes = [];

    /**
     * Create a new component attribute bag instance.
     *
     * @param  array  $attributes
     */
    public function __construct(array $attributes = [])
    {}

    /**
     * Get all the attribute values.
     *
     * @param  mixed  $keys
     * @return array
     */
    public function all($keys = null)
    {}

    /**
     * Get the first attribute's value.
     *
     * @param  mixed  $default
     * @return mixed
     */
    public function first($default = null)
    {}

    /**
     * Get a given attribute from the attribute array.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function get($key, $default = null)
    {}

    /**
     * Retrieve data from the instance.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return ($key is null ? array<array-key, mixed> : mixed)
     */
    protected function data($key = null, $default = null)
    {}

    /**
     * Only include the given attribute from the attribute array.
     *
     * @param  mixed  $keys
     * @return static
     */
    public function only($keys)
    {}

    /**
     * Exclude the given attribute from the attribute array.
     *
     * @param  mixed  $keys
     * @return static
     */
    public function except($keys)
    {}

    /**
     * Filter the attributes, returning a bag of attributes that pass the filter.
     *
     * @param  callable  $callback
     * @return static
     */
    public function filter($callback)
    {}

    /**
     * Return a bag of attributes that have keys starting with the given value / pattern.
     *
     * @param  string|string[]  $needles
     * @return static
     */
    public function whereStartsWith($needles)
    {}

    /**
     * Return a bag of attributes with keys that do not start with the given value / pattern.
     *
     * @param  string|string[]  $needles
     * @return static
     */
    public function whereDoesntStartWith($needles)
    {}

    /**
     * Return a bag of attributes that have keys starting with the given value / pattern.
     *
     * @param  string|string[]  $needles
     * @return static
     */
    public function thatStartWith($needles)
    {}

    /**
     * Only include the given attribute from the attribute array.
     *
     * @param  mixed  $keys
     * @return static
     */
    public function onlyProps($keys)
    {}

    /**
     * Exclude the given attribute from the attribute array.
     *
     * @param  mixed  $keys
     * @return static
     */
    public function exceptProps($keys)
    {}

    /**
     * Conditionally merge classes into the attribute bag.
     *
     * @param  mixed  $classList
     * @return static
     */
    public function class($classList)
    {}

    /**
     * Conditionally merge styles into the attribute bag.
     *
     * @param  mixed  $styleList
     * @return static
     */
    public function style($styleList)
    {}

    /**
     * Merge additional attributes / values into the attribute bag.
     *
     * @param  array  $attributeDefaults
     * @param  bool  $escape
     * @return static
     */
    public function merge(array $attributeDefaults = [], $escape = true)
    {}

    /**
     * Determine if the specific attribute value should be escaped.
     *
     * @param  bool  $escape
     * @param  mixed  $value
     * @return bool
     */
    protected function shouldEscapeAttributeValue($escape, $value)
    {}

    /**
     * Create a new appendable attribute value.
     *
     * @param  mixed  $value
     * @return \Illuminate\View\AppendableAttributeValue
     */
    public function prepends($value)
    {}

    /**
     * Resolve an appendable attribute value default value.
     *
     * @param  array  $attributeDefaults
     * @param  string  $key
     * @param  bool  $escape
     * @return mixed
     */
    protected function resolveAppendableAttributeDefault($attributeDefaults, $key, $escape)
    {}

    /**
     * Determine if the attribute bag is empty.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Determine if the attribute bag is not empty.
     *
     * @return bool
     */
    public function isNotEmpty()
    {}

    /**
     * Get all of the raw attributes.
     *
     * @return array
     */
    public function getAttributes()
    {}

    /**
     * Set the underlying attributes.
     *
     * @param  array  $attributes
     * @return void
     */
    public function setAttributes(array $attributes)
    {}

    /**
     * Extract "prop" names from given keys.
     *
     * @param  array  $keys
     * @return array
     */
    public static function extractPropNames(array $keys)
    {}

    /**
     * Get content as a string of HTML.
     *
     * @return string
     */
    public function toHtml()
    {}

    /**
     * Merge additional attributes / values into the attribute bag.
     *
     * @param  array  $attributeDefaults
     * @return \Illuminate\Support\HtmlString
     */
    public function __invoke(array $attributeDefaults = [])
    {}

    /**
     * Determine if the given offset exists.
     *
     * @param  string  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the value at the given offset.
     *
     * @param  string  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the value at a given offset.
     *
     * @param  string  $offset
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Remove the value at the given offset.
     *
     * @param  string  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Get an iterator for the items.
     *
     * @return \ArrayIterator
     */
    public function getIterator(): Traversable
    {}

    /**
     * Convert the object into a JSON serializable form.
     *
     * @return mixed
     */
    public function jsonSerialize(): mixed
    {}

    /**
     * Get all the attribute values.
     *
     * @return array
     */
    public function toArray()
    {}

    /**
     * Implode the attributes into a single HTML ready string.
     *
     * @return string
     */
    public function __toString()
    {}
}
