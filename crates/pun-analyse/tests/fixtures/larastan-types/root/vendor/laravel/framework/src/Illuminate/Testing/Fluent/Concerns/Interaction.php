<?php

namespace Illuminate\Testing\Fluent\Concerns;

use Illuminate\Support\Str;
use PHPUnit\Framework\Assert as PHPUnit;

trait Interaction
{
    /**
     * The list of interacted properties.
     *
     * @var array
     */
    protected $interacted = [];

    /**
     * Marks the property as interacted.
     *
     * @param  string  $key
     * @return void
     */
    protected function interactsWith(string $key): void
    {}

    /**
     * Asserts that all properties have been interacted with.
     *
     * @return void
     */
    public function interacted(): void
    {}

    /**
     * Disables the interaction check.
     *
     * @return $this
     */
    public function etc(): static
    {}

    /**
     * Retrieve a prop within the current scope using "dot" notation.
     *
     * @param  string|null  $key
     * @return mixed
     */
    abstract protected function prop(?string $key = null);
}
