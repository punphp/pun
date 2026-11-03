<?php

namespace Illuminate\View;

use Illuminate\Contracts\Support\Htmlable;
use InvalidArgumentException;
use Stringable;

class ComponentSlot implements Htmlable, Stringable
{
    /**
     * The slot attribute bag.
     *
     * @var \Illuminate\View\ComponentAttributeBag
     */
    public $attributes;

    /**
     * The slot contents.
     *
     * @var string
     */
    protected $contents;

    /**
     * Create a new slot instance.
     *
     * @param  string  $contents
     * @param  array  $attributes
     */
    public function __construct($contents = '', $attributes = [])
    {}

    /**
     * Set the extra attributes that the slot should make available.
     *
     * @param  array  $attributes
     * @return $this
     */
    public function withAttributes(array $attributes)
    {}

    /**
     * Get the slot's HTML string.
     *
     * @return string
     */
    public function toHtml()
    {}

    /**
     * Determine if the slot is empty.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Determine if the slot is not empty.
     *
     * @return bool
     */
    public function isNotEmpty()
    {}

    /**
     * Determine if the slot has non-comment content.
     *
     * @param  callable|string|null  $callable
     * @return bool
     *
     * @throws \InvalidArgumentException
     */
    public function hasActualContent(callable|string|null $callable = null)
    {}

    /**
     * Get the slot's HTML string.
     *
     * @return string
     */
    public function __toString()
    {}
}
