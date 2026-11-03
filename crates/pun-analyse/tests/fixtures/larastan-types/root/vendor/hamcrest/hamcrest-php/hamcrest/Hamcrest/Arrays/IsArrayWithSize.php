<?php
namespace Hamcrest\Arrays;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Core\DescribedAs;
use Hamcrest\Core\IsNot;
use Hamcrest\FeatureMatcher;
use Hamcrest\Matcher;
use Hamcrest\Util;

/**
 * Matches if array size satisfies a nested matcher.
 */
class IsArrayWithSize extends FeatureMatcher
{

    public function __construct(Matcher $sizeMatcher)
    {}

    protected function featureValueOf($array)
    {}

    /**
     * Does array size satisfy a given matcher?
     *
     * @param \Hamcrest\Matcher|int $size as a {@link Hamcrest\Matcher} or a value.
     *
     * @return \Hamcrest\Arrays\IsArrayWithSize
     * @factory
     */
    public static function arrayWithSize($size): self
    {}

    /**
     * Matches an empty array.
     *
     * @factory
     */
    public static function emptyArray(): DescribedAs
    {}

    /**
     * Matches an empty array.
     *
     * @factory
     */
    public static function nonEmptyArray(): DescribedAs
    {}
}
