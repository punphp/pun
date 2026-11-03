<?php
namespace Hamcrest\Text;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Core\AnyOf;
use Hamcrest\Core\IsNull;
use Hamcrest\Description;

/**
 * Matches empty Strings (and null).
 */
class IsEmptyString extends BaseMatcher
{

    private static ?self $_INSTANCE = null;
    private static ?AnyOf $_NULL_OR_EMPTY_INSTANCE = null;
    private static ?self $_NOT_INSTANCE = null;

    private bool $_empty;

    public function __construct(bool $empty = true)
    {}

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Matches if value is a zero-length string.
     *
     * @factory emptyString
     */
    public static function isEmptyString(): self
    {}

    /**
     * Matches if value is null or a zero-length string.
     *
     * @factory nullOrEmptyString
     */
    public static function isEmptyOrNullString(): AnyOf
    {}

    /**
     * Matches if value is a non-zero-length string.
     *
     * @factory nonEmptyString
     */
    public static function isNonEmptyString(): self
    {}
}
