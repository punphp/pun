<?php

namespace Illuminate\Support;

use Illuminate\Contracts\Support\Htmlable;
use Stringable;

class HtmlString implements Htmlable, Stringable
{
    /**
     * The HTML string.
     *
     * @var string
     */
    protected $html;

    /**
     * Create a new HTML string instance.
     *
     * @param  string  $html
     */
    public function __construct($html = '')
    {}

    /**
     * Get the HTML string.
     *
     * @return string
     */
    public function toHtml()
    {}

    /**
     * Determine if the given HTML string is empty.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Determine if the given HTML string is not empty.
     *
     * @return bool
     */
    public function isNotEmpty()
    {}

    /**
     * Get the HTML string.
     *
     * @return string
     */
    public function __toString()
    {}
}
