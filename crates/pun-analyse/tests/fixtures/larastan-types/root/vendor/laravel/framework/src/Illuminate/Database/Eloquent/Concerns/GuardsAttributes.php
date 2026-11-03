<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Illuminate\Database\Eloquent\Attributes\Fillable;
use Illuminate\Database\Eloquent\Attributes\Guarded;
use Illuminate\Database\Eloquent\Attributes\Initialize;
use Illuminate\Database\Eloquent\Attributes\Unguarded;
use Illuminate\Database\Eloquent\Relations\Pivot;

trait GuardsAttributes
{
    /**
     * The attributes that are mass assignable.
     *
     * @var array<int, string>
     */
    protected $fillable = [];

    /**
     * The attributes that aren't mass assignable.
     *
     * @var array<string>
     */
    protected $guarded = ['*'];

    /**
     * Indicates if all mass assignment is enabled.
     *
     * @var bool
     */
    protected static $unguarded = false;

    /**
     * The actual columns that exist on the database and can be guarded.
     *
     * @var array<class-string,list<string>>
     */
    protected static $guardableColumns = [];

    /**
     * Initialize the GuardsAttributes trait.
     *
     * @return void
     */
    #[Initialize]
    public function initializeGuardsAttributes()
    {}

    /**
     * Get the fillable attributes for the model.
     *
     * @return array<string>
     */
    public function getFillable()
    {}

    /**
     * Set the fillable attributes for the model.
     *
     * @param  array<string>  $fillable
     * @return $this
     */
    public function fillable(array $fillable)
    {}

    /**
     * Merge new fillable attributes with existing fillable attributes on the model.
     *
     * @param  array<string>  $fillable
     * @return $this
     */
    public function mergeFillable(array $fillable)
    {}

    /**
     * Get the guarded attributes for the model.
     *
     * @return array<string>
     */
    public function getGuarded()
    {}

    /**
     * Set the guarded attributes for the model.
     *
     * @param  array<string>  $guarded
     * @return $this
     */
    public function guard(array $guarded)
    {}

    /**
     * Merge new guarded attributes with existing guarded attributes on the model.
     *
     * @param  array<string>  $guarded
     * @return $this
     */
    public function mergeGuarded(array $guarded)
    {}

    /**
     * Disable all mass assignable restrictions.
     *
     * @param  bool  $state
     * @return void
     */
    public static function unguard($state = true)
    {}

    /**
     * Enable the mass assignment restrictions.
     *
     * @return void
     */
    public static function reguard()
    {}

    /**
     * Determine if the current state is "unguarded".
     *
     * @return bool
     */
    public static function isUnguarded()
    {}

    /**
     * Run the given callable while being unguarded.
     *
     * @template TReturn
     *
     * @param  callable(): TReturn  $callback
     * @return TReturn
     */
    public static function unguarded(callable $callback)
    {}

    /**
     * Determine if the given attribute may be mass assigned.
     *
     * @param  string  $key
     * @return bool
     */
    public function isFillable($key)
    {}

    /**
     * Determine if the given key is guarded.
     *
     * @param  string  $key
     * @return bool
     */
    public function isGuarded($key)
    {}

    /**
     * Determine if the given column is a valid, guardable column.
     *
     * @param  string  $key
     * @return bool
     */
    protected function isGuardableColumn($key)
    {}

    /**
     * Determine if the model is totally guarded.
     *
     * @return bool
     */
    public function totallyGuarded()
    {}

    /**
     * Get the fillable attributes of a given array.
     *
     * @param  array<string, mixed>  $attributes
     * @return array<string, mixed>
     */
    protected function fillableFromArray(array $attributes)
    {}
}
