<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;

/**
 * Is the value equal to another value, as tested by the use of the "=="
 * comparison operator?
 */
class IsEqual extends BaseMatcher
{
    /**
     * @var mixed
     */
    private $_item;

    /**
     * @param mixed $item
     */
    public function __construct($item)
    {}

    public function matches($arg): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Is the value equal to another value, as tested by the use of the "=="
     * comparison operator?
     *
     * @factory
     * @param mixed $item
     */
    public static function equalTo($item): self
    {}
}
