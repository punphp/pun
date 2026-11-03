<?php
namespace Hamcrest\Type;

/*
 Copyright (c) 2010 hamcrest.org
 */
use Hamcrest\Core\IsTypeOf;

/**
 * Tests whether the value is a resource.
 */
class IsResource extends IsTypeOf
{

    /**
     * Creates a new instance of IsResource
     */
    public function __construct()
    {}

    /**
     * Is the value a resource?
     *
     * @factory
     */
    public static function resourceValue(): self
    {}
}
