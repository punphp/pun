<?php

namespace Illuminate\View\Concerns;

use Illuminate\Support\Arr;
use Illuminate\Support\LazyCollection;

trait ManagesLoops
{
    /**
     * The stack of in-progress loops.
     *
     * @var array
     */
    protected $loopsStack = [];

    /**
     * Add new loop to the stack.
     *
     * @param  \Countable|array  $data
     * @return void
     */
    public function addLoop($data)
    {}

    /**
     * Increment the top loop's indices.
     *
     * @return void
     */
    public function incrementLoopIndices()
    {}

    /**
     * Pop a loop from the top of the loop stack.
     *
     * @return void
     */
    public function popLoop()
    {}

    /**
     * Get an instance of the last loop in the stack.
     *
     * @return \stdClass|null
     */
    public function getLastLoop()
    {}

    /**
     * Get the entire loop stack.
     *
     * @return array
     */
    public function getLoopStack()
    {}
}
