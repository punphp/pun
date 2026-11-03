<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2010 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;

/**
 * Tests if a value (class, object, or array) has a named property.
 *
 * For example:
 * <pre>
 * assertThat(array('a', 'b'), set('b'));
 * assertThat($foo, set('bar'));
 * assertThat('Server', notSet('defaultPort'));
 * </pre>
 *
 * @todo Replace $property with a matcher and iterate all property names.
 */
class Set extends BaseMatcher
{

    /**
     * @var mixed $_property
     */
    private $_property;
    private bool $_not;

    /**
     * @param mixed $property
     */
    public function __construct($property, bool $not = false)
    {}

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    public function describeMismatch($item, Description $description): void
    {}

    /**
     * Matches if value (class, object, or array) has named $property.
     *
     * @factory
     * @param mixed $property
     */
    public static function set($property): self
    {}

    /**
     * Matches if value (class, object, or array) does not have named $property.
     *
     * @factory
     * @param mixed $property
     */
    public static function notSet($property): self
    {}
}
