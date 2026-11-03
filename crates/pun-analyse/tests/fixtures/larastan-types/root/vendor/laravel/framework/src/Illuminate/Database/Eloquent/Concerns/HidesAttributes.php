<?php

namespace Illuminate\Database\Eloquent\Concerns;

use Illuminate\Database\Eloquent\Attributes\Hidden;
use Illuminate\Database\Eloquent\Attributes\Initialize;
use Illuminate\Database\Eloquent\Attributes\Visible;

trait HidesAttributes
{
    /**
     * The attributes that should be hidden for serialization.
     *
     * @var array<string>
     */
    protected $hidden = [];

    /**
     * The attributes that should be visible in serialization.
     *
     * @var array<string>
     */
    protected $visible = [];

    /**
     * Initialize the HidesAttributes trait.
     *
     * @return void
     */
    #[Initialize]
    public function initializeHidesAttributes()
    {}

    /**
     * Get the hidden attributes for the model.
     *
     * @return array<string>
     */
    public function getHidden()
    {}

    /**
     * Set the hidden attributes for the model.
     *
     * @param  array<string>  $hidden
     * @return $this
     */
    public function setHidden(array $hidden)
    {}

    /**
     * Merge new hidden attributes with existing hidden attributes on the model.
     *
     * @param  array<string>  $hidden
     * @return $this
     */
    public function mergeHidden(array $hidden)
    {}

    /**
     * Get the visible attributes for the model.
     *
     * @return array<string>
     */
    public function getVisible()
    {}

    /**
     * Set the visible attributes for the model.
     *
     * @param  array<string>  $visible
     * @return $this
     */
    public function setVisible(array $visible)
    {}

    /**
     * Merge new visible attributes with existing visible attributes on the model.
     *
     * @param  array<string>  $visible
     * @return $this
     */
    public function mergeVisible(array $visible)
    {}

    /**
     * Make the given, typically hidden, attributes visible.
     *
     * @param  array<string>|string|null  $attributes
     * @return $this
     */
    public function makeVisible($attributes)
    {}

    /**
     * Make the given, typically hidden, attributes visible if the given truth test passes.
     *
     * @param  bool|\Closure  $condition
     * @param  array<string>|string|null  $attributes
     * @return $this
     */
    public function makeVisibleIf($condition, $attributes)
    {}

    /**
     * Make the given, typically visible, attributes hidden.
     *
     * @param  array<string>|string|null  $attributes
     * @return $this
     */
    public function makeHidden($attributes)
    {}

    /**
     * Make the given, typically visible, attributes hidden if the given truth test passes.
     *
     * @param  bool|\Closure  $condition
     * @param  array<string>|string|null  $attributes
     * @return $this
     */
    public function makeHiddenIf($condition, $attributes)
    {}
}
