<?php

namespace Illuminate\Testing\Fluent;

use Closure;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\Tappable;
use Illuminate\Testing\AssertableJsonString;
use PHPUnit\Framework\Assert as PHPUnit;

class AssertableJson implements Arrayable
{
    use Concerns\Has,
        Concerns\Matching,
        Concerns\Debugging,
        Concerns\Interaction,
        Conditionable,
        Macroable,
        Tappable;

    /**
     * The properties in the current scope.
     *
     * @var array
     */
    private $props;

    /**
     * The "dot" path to the current scope.
     *
     * @var string|null
     */
    private $path;

    /**
     * Create a new fluent, assertable JSON data instance.
     *
     * @param  array  $props
     * @param  string|null  $path
     */
    protected function __construct(array $props, ?string $path = null)
    {}

    /**
     * Compose the absolute "dot" path to the given key.
     *
     * @param  string  $key
     * @return string
     */
    protected function dotPath(string $key = ''): string
    {}

    /**
     * Retrieve a prop within the current scope using "dot" notation.
     *
     * @param  string|null  $key
     * @return mixed
     */
    protected function prop(?string $key = null)
    {}

    /**
     * Instantiate a new "scope" at the path of the given key.
     *
     * @param  string  $key
     * @param  \Closure  $callback
     * @return $this
     */
    protected function scope(string $key, Closure $callback): static
    {}

    /**
     * Instantiate a new "scope" on the first child element.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function first(Closure $callback): static
    {}

    /**
     * Instantiate a new "scope" on each child element.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function each(Closure $callback): static
    {}

    /**
     * Create a new instance from an array.
     *
     * @param  array  $data
     * @return static
     */
    public static function fromArray(array $data): static
    {}

    /**
     * Create a new instance from an AssertableJsonString.
     *
     * @param  \Illuminate\Testing\AssertableJsonString  $json
     * @return static
     */
    public static function fromAssertableJsonString(AssertableJsonString $json): static
    {}

    /**
     * Get the instance as an array.
     *
     * @return array
     */
    public function toArray()
    {}
}
