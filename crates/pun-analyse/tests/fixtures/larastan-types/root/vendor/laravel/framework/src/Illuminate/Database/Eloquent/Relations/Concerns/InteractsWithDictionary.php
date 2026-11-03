<?php

namespace Illuminate\Database\Eloquent\Relations\Concerns;

use InvalidArgumentException;
use UnitEnum;

use function Illuminate\Support\enum_value;

trait InteractsWithDictionary
{
    /**
     * Get a dictionary key attribute - casting it to a string if necessary.
     *
     * @param  mixed  $attribute
     * @return string|int|null
     *
     * @throws \InvalidArgumentException
     */
    protected function getDictionaryKey($attribute)
    {}
}
