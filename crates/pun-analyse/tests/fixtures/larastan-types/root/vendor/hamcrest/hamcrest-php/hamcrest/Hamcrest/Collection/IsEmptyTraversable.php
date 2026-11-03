<?php
namespace Hamcrest\Collection;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;

/**
 * Matches if traversable is empty or non-empty.
 */
class IsEmptyTraversable extends BaseMatcher
{

    private static ?self $_INSTANCE = null;
    private static ?self $_NOT_INSTANCE = null;

    private bool $_empty;

    public function __construct(bool $empty = true)
    {}

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Returns true if traversable is empty.
     *
     * @factory
     */
    public static function emptyTraversable(): self
    {}

    /**
     * Returns true if traversable is not empty.
     *
     * @factory
     */
    public static function nonEmptyTraversable(): self
    {}
}
