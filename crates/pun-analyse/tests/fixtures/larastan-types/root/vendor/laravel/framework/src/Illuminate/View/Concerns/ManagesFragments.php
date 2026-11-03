<?php

namespace Illuminate\View\Concerns;

use InvalidArgumentException;

trait ManagesFragments
{
    /**
     * All of the captured, rendered fragments.
     *
     * @var array
     */
    protected $fragments = [];

    /**
     * The stack of in-progress fragment renders.
     *
     * @var array
     */
    protected $fragmentStack = [];

    /**
     * Start injecting content into a fragment.
     *
     * @param  string  $fragment
     * @return void
     */
    public function startFragment($fragment)
    {}

    /**
     * Stop injecting content into a fragment.
     *
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function stopFragment()
    {}

    /**
     * Get the contents of a fragment.
     *
     * @param  string  $name
     * @param  string|null  $default
     * @return mixed
     */
    public function getFragment($name, $default = null)
    {}

    /**
     * Get the entire array of rendered fragments.
     *
     * @return array
     */
    public function getFragments()
    {}

    /**
     * Flush all of the fragments.
     *
     * @return void
     */
    public function flushFragments()
    {}
}
