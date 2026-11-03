<?php
namespace Hamcrest;

/*
 Copyright (c) 2009 hamcrest.org
 */

/**
 * A {@link Hamcrest\Description} that is stored as a string.
 */
class StringDescription extends BaseDescription
{

    private string $_out;

    /**
     * @param mixed $out
     */
    public function __construct($out = '')
    {}

    public function __toString()
    {}

    /**
     * Return the description of a {@link Hamcrest\SelfDescribing} object as a
     * String.
     *
     * @param \Hamcrest\SelfDescribing $selfDescribing
     *   The object to be described.
     *
     * @return string
     *   The description of the object.
     */
    public static function toString(SelfDescribing $selfDescribing): string
    {}

    /**
     * Alias for {@link toString()}.
     */
    public static function asString(SelfDescribing $selfDescribing): string
    {}

    // -- Protected Methods

    protected function append($str): void
    {}
}
