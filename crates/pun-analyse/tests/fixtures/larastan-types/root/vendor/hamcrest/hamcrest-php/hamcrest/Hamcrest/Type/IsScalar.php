<?php
namespace Hamcrest\Type;

/*
 Copyright (c) 2010 hamcrest.org
 */
use Hamcrest\Core\IsTypeOf;

/**
 * Tests whether the value is a scalar (boolean, integer, double, or string).
 */
class IsScalar extends IsTypeOf
{

    public function __construct()
    {}

    public function matches($item): bool
    {}

    /**
     * Is the value a scalar (boolean, integer, double, or string)?
     *
     * @factory
     */
    public static function scalarValue(): self
    {}
}
