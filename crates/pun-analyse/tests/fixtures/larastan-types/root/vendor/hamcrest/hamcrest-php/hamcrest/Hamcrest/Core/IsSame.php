<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;

/**
 * Is the value the same object as another value?
 * In PHP terms, does $a === $b?
 */
class IsSame extends BaseMatcher
{

    /**
     * @var mixed object
     */
    private $_object;

    /**
     * @param mixed $object
     */
    public function __construct($object)
    {}

    public function matches($object): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Creates a new instance of IsSame.
     *
     * @param mixed $object
     *   The predicate evaluates to true only when the argument is
     *   this object.
     *
     * @return \Hamcrest\Core\IsSame
     * @factory
     */
    public static function sameInstance($object): self
    {}
}
