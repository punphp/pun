<?php
namespace Hamcrest;

/*
 Copyright (c) 2009 hamcrest.org
 */

/**
 * BaseClass for all Matcher implementations.
 *
 * @see Hamcrest\Matcher
 */
abstract class BaseMatcher implements Matcher
{
    /**
     * @param mixed $item
     */
    public function describeMismatch($item, Description $description): void
    {}

    public function __toString(): string
    {}

    public function __invoke(): bool
    { func_get_args(); }
}
