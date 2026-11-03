<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2010 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;

/**
 * Tests whether the value has a built-in type.
 */
class IsTypeOf extends BaseMatcher
{

    private string $_theType;

    /**
     * Creates a new instance of IsTypeOf
     *
     * @param string $theType
     *   The predicate evaluates to true for values with this built-in type.
     */
    public function __construct(string $theType)
    {}

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    public function describeMismatch($item, Description $description): void
    {}

    public static function getTypeDescription(string $type): string
    {}

    /**
     * Is the value a particular built-in type?
     *
     * @factory
     * @param string $theType
     */
    public static function typeOf(string $theType): self
    {}
}
