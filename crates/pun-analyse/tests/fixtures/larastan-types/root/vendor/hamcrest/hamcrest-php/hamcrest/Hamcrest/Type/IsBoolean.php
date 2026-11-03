<?php
namespace Hamcrest\Type;

/*
 Copyright (c) 2010 hamcrest.org
 */
use Hamcrest\Core\IsTypeOf;

/**
 * Tests whether the value is a boolean.
 */
class IsBoolean extends IsTypeOf
{

    /**
     * Creates a new instance of IsBoolean
     */
    public function __construct()
    {}

    /**
     * Is the value a boolean?
     *
     * @factory boolValue
     */
    public static function booleanValue(): self
    {}
}
