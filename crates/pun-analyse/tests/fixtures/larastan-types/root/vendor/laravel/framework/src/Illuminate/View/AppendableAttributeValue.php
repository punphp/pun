<?php

namespace Illuminate\View;

use Stringable;

class AppendableAttributeValue implements Stringable
{
    /**
     * The attribute value.
     *
     * @var mixed
     */
    public $value;

    /**
     * Create a new appendable attribute value.
     *
     * @param  mixed  $value
     */
    public function __construct($value)
    {}

    /**
     * Get the string value.
     *
     * @return string
     */
    public function __toString()
    {}
}
