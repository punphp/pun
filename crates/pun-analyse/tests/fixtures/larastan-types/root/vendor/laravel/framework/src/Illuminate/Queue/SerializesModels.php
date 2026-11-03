<?php

namespace Illuminate\Queue;

use Illuminate\Queue\Attributes\WithoutRelations;
use Illuminate\Support\Reflector;
use ReflectionClass;
use ReflectionProperty;

trait SerializesModels
{
    use SerializesAndRestoresModelIdentifiers;

    /**
     * Prepare the instance values for serialization.
     *
     * @return array
     */
    public function __serialize()
    {}

    /**
     * Restore the model after serialization.
     *
     * @param  array  $values
     * @return void
     */
    public function __unserialize(array $values)
    {}

    /**
     * Get the property value for the given property.
     *
     * @param  \ReflectionProperty  $property
     * @return mixed
     */
    protected function getPropertyValue(ReflectionProperty $property)
    {}
}
