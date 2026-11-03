<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;

/**
 * A matcher that always returns <code>true</code>.
 */
class IsAnything extends BaseMatcher
{

    private string $_message;

    public function __construct(string $message = 'ANYTHING')
    {}

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * This matcher always evaluates to true.
     *
     * @param string $description A meaningful string used when describing itself.
     *
     * @factory
     */
    public static function anything(string $description = 'ANYTHING'): self
    {}
}
