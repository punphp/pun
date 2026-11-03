<?php

namespace Illuminate\Database\Eloquent\Attributes;

use Attribute;

#[Attribute(Attribute::TARGET_CLASS)]
class Touches
{
    /**
     * @var array<int, string>
     */
    public array $relations;

    /**
     * Create a new attribute instance.
     *
     * @param  array<int, string>|string  ...$relations
     */
    public function __construct(array|string ...$relations)
    {}
}
