<?php

namespace Illuminate\Database\Eloquent\Casts;

class Attribute
{
    /**
     * The attribute accessor.
     *
     * @var callable|null
     */
    public $get;

    /**
     * The attribute mutator.
     *
     * @var callable|null
     */
    public $set;

    /**
     * Indicates if caching is enabled for this attribute.
     *
     * @var bool
     */
    public $withCaching = false;

    /**
     * Indicates if caching of objects is enabled for this attribute.
     *
     * @var bool
     */
    public $withObjectCaching = true;

    /**
     * Create a new attribute accessor / mutator.
     *
     * @param  callable|null  $get
     * @param  callable|null  $set
     */
    public function __construct(?callable $get = null, ?callable $set = null)
    {}

    /**
     * Create a new attribute accessor / mutator.
     *
     * @param  callable|null  $get
     * @param  callable|null  $set
     * @return static
     */
    public static function make(?callable $get = null, ?callable $set = null): static
    {}

    /**
     * Create a new attribute accessor.
     *
     * @param  callable  $get
     * @return static
     */
    public static function get(callable $get)
    {}

    /**
     * Create a new attribute mutator.
     *
     * @param  callable  $set
     * @return static
     */
    public static function set(callable $set)
    {}

    /**
     * Disable object caching for the attribute.
     *
     * @return $this
     */
    public function withoutObjectCaching()
    {}

    /**
     * Enable caching for the attribute.
     *
     * @return $this
     */
    public function shouldCache()
    {}
}
