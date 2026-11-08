<?php

namespace Illuminate\Support\Traits;

use Carbon\CarbonInterval;
use Carbon\Unit;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\Number;
use Illuminate\Support\Stringable;
use stdClass;

use function Illuminate\Support\enum_value;

trait InteractsWithData
{
    /**
     * Retrieve all data from the instance.
     *
     * @param  mixed  $keys
     * @return array
     */
    abstract public function all($keys = null);

    /**
     * Retrieve data from the instance.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    abstract protected function data($key = null, $default = null);

    /**
     * Determine if the data contains a given key.
     *
     * @param  string|array  $key
     * @return bool
     */
    public function exists($key)
    {}

    /**
     * Determine if the data contains a given key.
     *
     * @param  string|array  $key
     * @return bool
     */
    public function has($key)
    { func_get_args(); }

    /**
     * Determine if the instance contains any of the given keys.
     *
     * @param  string|array  $keys
     * @return bool
     */
    public function hasAny($keys)
    { func_get_args(); }

    /**
     * Apply the callback if the instance contains the given key.
     *
     * @template TReturn
     * @template TReturnDefault = never
     *
     * @param  string  $key
     * @param  callable(mixed): TReturn  $callback
     * @param  (callable(): TReturnDefault)|null  $default
     * @return $this|TReturn|TReturnDefault
     */
    public function whenHas($key, callable $callback, ?callable $default = null)
    {}

    /**
     * Determine if the instance contains a non-empty value for the given key.
     *
     * @param  string|array  $key
     * @return bool
     */
    public function filled($key)
    { func_get_args(); }

    /**
     * Determine if the instance contains an empty value for the given key.
     *
     * @param  string|array  $key
     * @return bool
     */
    public function isNotFilled($key)
    { func_get_args(); }

    /**
     * Determine if the instance contains a non-empty value for any of the given keys.
     *
     * @param  string|array  $keys
     * @return bool
     */
    public function anyFilled($keys)
    { func_get_args(); }

    /**
     * Apply the callback if the instance contains a non-empty value for the given key.
     *
     * @template TReturn
     * @template TReturnDefault = never
     *
     * @param  string  $key
     * @param  callable(mixed): TReturn  $callback
     * @param  (callable(): TReturnDefault)|null  $default
     * @return $this|TReturn|TReturnDefault
     */
    public function whenFilled($key, callable $callback, ?callable $default = null)
    {}

    /**
     * Apply the callback if the instance contains a valid enum value for the given key.
     *
     * @template TEnum of \BackedEnum
     * @template TReturn
     * @template TReturnDefault = never
     *
     * @param  string  $key
     * @param  class-string<TEnum>  $enumClass
     * @param  callable(TEnum):TReturn  $callback
     * @param  (callable(): TReturnDefault)|null  $default
     * @return $this|TReturn|TReturnDefault
     */
    public function whenEnum($key, string $enumClass, callable $callback, ?callable $default = null)
    {}

    /**
     * Determine if the instance is missing a given key.
     *
     * @param  string|array  $key
     * @return bool
     */
    public function missing($key)
    { func_get_args(); }

    /**
     * Apply the callback if the instance is missing the given key.
     *
     * @template TReturn
     * @template TReturnDefault = never
     *
     * @param  string  $key
     * @param  callable(mixed): TReturn  $callback
     * @param  (callable(): TReturnDefault)|null  $default
     * @return $this|TReturn|TReturnDefault
     */
    public function whenMissing($key, callable $callback, ?callable $default = null)
    {}

    /**
     * Determine if the given key is an empty string for "filled".
     *
     * @param  string  $key
     * @return bool
     */
    protected function isEmptyString($key)
    {}

    /**
     * Retrieve data from the instance as a Stringable instance.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @param  bool  $strict
     * @return \Illuminate\Support\Stringable
     */
    public function str($key, $default = null, $strict = false)
    {}

    /**
     * Retrieve data from the instance as a Stringable instance.
     *
     * When strict, values that are not strings, numbers, or Stringable objects return the default.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @param  bool  $strict
     * @return \Illuminate\Support\Stringable
     */
    public function string($key, $default = null, $strict = false)
    {}

    /**
     * Retrieve data as a boolean value.
     *
     * Returns true when value is "1", "true", "on", and "yes". Otherwise, returns false.
     * When strict, values other than "1", "true", "on", "yes", "0", "false", "off", and "no" return the default.
     *
     * @param  string|null  $key
     * @param  bool  $default
     * @param  bool  $strict
     * @return bool
     */
    public function boolean($key = null, $default = false, $strict = false)
    {}

    /**
     * Retrieve data as an integer value.
     *
     * When strict, values that are not valid integers return the default.
     *
     * @param  string  $key
     * @param  int  $default
     * @param  bool  $strict
     * @return int
     */
    public function integer($key, $default = 0, $strict = false)
    {}

    /**
     * Retrieve data as a float value.
     *
     * When strict, values that are not valid numbers return the default.
     *
     * @param  string  $key
     * @param  float  $default
     * @param  bool  $strict
     * @return float
     */
    public function float($key, $default = 0.0, $strict = false)
    {}

    /**
     * Retrieve data clamped between min and max values.
     *
     * @param  string  $key
     * @param  int|float  $min
     * @param  int|float  $max
     * @param  int|float  $default
     * @return float|int
     */
    public function clamp($key, $min, $max, $default = 0)
    {}

    /**
     * Retrieve data from the instance as a Carbon instance.
     *
     * @param  string  $key
     * @param  string|null  $format
     * @param  \UnitEnum|string|null  $tz
     * @return \Illuminate\Support\Carbon|null
     *
     * @throws \Carbon\Exceptions\InvalidFormatException
     */
    public function date($key, $format = null, $tz = null)
    {}

    /**
     * Retrieve data from the instance as a CarbonInterval instance.
     *
     * @param  string  $key
     * @param  \Carbon\Unit|string|null  $unit
     * @return \Carbon\CarbonInterval|null
     */
    public function interval($key, $unit = null)
    {}

    /**
     * Retrieve data from the instance as an enum.
     *
     * @template TEnum of \BackedEnum
     * @template TDefault of TEnum|null
     *
     * @param  string  $key
     * @param  class-string<TEnum>  $enumClass
     * @param  TDefault  $default
     * @return TEnum|TDefault
     */
    public function enum($key, $enumClass, $default = null)
    {}

    /**
     * Retrieve data from the instance as an array of enums.
     *
     * @template TEnum of \BackedEnum
     *
     * @param  string  $key
     * @param  class-string<TEnum>  $enumClass
     * @return TEnum[]
     */
    public function enums($key, $enumClass)
    {}

    /**
     * Determine if the given enum class is backed.
     *
     * @param  class-string  $enumClass
     * @return bool
     */
    protected function isBackedEnum($enumClass)
    {}

    /**
     * Retrieve data from the instance as an array.
     *
     * @param  array|string|null  $key
     * @return array
     */
    public function array($key = null)
    {}

    /**
     * Retrieve data from the instance as a collection.
     *
     * @param  array|string|null  $key
     * @return \Illuminate\Support\Collection
     */
    public function collect($key = null)
    {}

    /**
     * Get a subset containing the provided keys with values from the instance data.
     *
     * @param  mixed  $keys
     * @return array
     */
    public function only($keys)
    { func_get_args(); }

    /**
     * Get all of the data except for a specified array of items.
     *
     * @param  mixed  $keys
     * @return array
     */
    public function except($keys)
    { func_get_args(); }
}
