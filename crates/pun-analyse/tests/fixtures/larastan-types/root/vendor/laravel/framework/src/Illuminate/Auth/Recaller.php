<?php

namespace Illuminate\Auth;

class Recaller
{
    /**
     * The "recaller" / "remember me" cookie string.
     *
     * @var string
     */
    protected $recaller;

    /**
     * Create a new recaller instance.
     *
     * @param  string  $recaller
     */
    public function __construct($recaller)
    {}

    /**
     * Get the user ID from the recaller.
     *
     * @return string
     */
    public function id()
    {}

    /**
     * Get the "remember token" token from the recaller.
     *
     * @return string
     */
    public function token()
    {}

    /**
     * Get the password from the recaller.
     *
     * @return string
     */
    public function hash()
    {}

    /**
     * Determine if the recaller is valid.
     *
     * @return bool
     */
    public function valid()
    {}

    /**
     * Determine if the recaller is an invalid string.
     *
     * @return bool
     */
    protected function properString()
    {}

    /**
     * Determine if the recaller has all segments.
     *
     * @return bool
     */
    protected function hasAllSegments()
    {}

    /**
     * Get the recaller's segments.
     *
     * @return array
     */
    public function segments()
    {}
}
