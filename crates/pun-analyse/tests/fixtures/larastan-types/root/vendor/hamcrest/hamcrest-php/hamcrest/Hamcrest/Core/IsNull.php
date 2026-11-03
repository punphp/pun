<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;

/**
 * Is the value null?
 */
class IsNull extends BaseMatcher
{

    private static ?self $_INSTANCE = null;
    private static ?IsNot $_NOT_INSTANCE = null;

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Matches if value is null.
     *
     * @factory
     */
    public static function nullValue(): self
    {}

    /**
     * Matches if value is not null.
     *
     * @factory
     */
    public static function notNullValue(): IsNot
    {}
}
