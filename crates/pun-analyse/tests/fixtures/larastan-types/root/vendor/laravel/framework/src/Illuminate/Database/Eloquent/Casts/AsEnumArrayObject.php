<?php

namespace Illuminate\Database\Eloquent\Casts;

use BackedEnum;
use Illuminate\Contracts\Database\Eloquent\Castable;
use Illuminate\Contracts\Database\Eloquent\CastsAttributes;
use Illuminate\Support\Collection;

use function Illuminate\Support\enum_value;

class AsEnumArrayObject implements Castable
{
    /**
     * Get the caster class to use when casting from / to this cast target.
     *
     * @template TEnum of \UnitEnum
     *
     * @param  array{class-string<TEnum>}  $arguments
     * @return \Illuminate\Contracts\Database\Eloquent\CastsAttributes<\Illuminate\Database\Eloquent\Casts\ArrayObject<array-key, TEnum>, iterable<TEnum>>
     */
    public static function castUsing(array $arguments)
    {}

    /**
     * Specify the Enum for the cast.
     *
     * @param  class-string  $class
     * @return string
     */
    public static function of($class)
    {}
}
