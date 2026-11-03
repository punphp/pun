<?php
namespace Hamcrest\Type;

/*
 Copyright (c) 2010 hamcrest.org
 */
use Hamcrest\Core\IsTypeOf;

/**
 * Tests whether the value is numeric.
 */
class IsNumeric extends IsTypeOf
{

    public function __construct()
    {}

    public function matches($item): bool
    {}

    /**
     * Return if the string passed is a valid hexadecimal number.
     * This check is necessary because PHP 7 doesn't recognize hexadecimal string as numeric anymore.
     *
     * @param mixed $item
     * @return boolean
     */
    private function isHexadecimal($item)
    {}

    /**
     * Is the value a numeric?
     *
     * @factory
     */
    public static function numericValue(): self
    {}
}
