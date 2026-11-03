<?php

namespace Illuminate\Support\Traits;

use Exception;
use ReflectionClass;

trait ReadsClassAttributes
{
    /**
     * Get a configuration value from an attribute, falling back to a property.
     *
     * @param  object  $target
     * @param  class-string  $attributeClass
     * @param  string|null  $property
     * @param  mixed  $default
     * @return mixed
     */
    protected function getAttributeValue($target, string $attributeClass, ?string $property = null, $default = null)
    {}

    /**
     * Extract the value from an attribute instance.
     *
     * @param  object  $instance
     * @return mixed
     */
    protected function extractAttributeValue($instance)
    {}

    /**
     * Get an instance of the given attribute class from the target class or its parents.
     *
     * @param  object  $target
     * @param  class-string  $attributeClass
     * @param  \ReflectionClass|null  $declaringClass
     * @return object|null
     */
    protected function getAttributeInstance($target, string $attributeClass, ?ReflectionClass &$declaringClass = null)
    {}

    /**
     * Determine if a property declared on a child class overrides an inherited attribute.
     *
     * @param  object  $target
     * @param  \ReflectionClass  $reflection
     * @param  string|null  $property
     * @param  \ReflectionClass  $attributeDeclaringClass
     * @return bool
     */
    protected function propertyOverridesAttribute($target, ReflectionClass $reflection, ?string $property, ReflectionClass $attributeDeclaringClass)
    {}
}
