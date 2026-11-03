<?php
namespace Hamcrest\Collection;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\FeatureMatcher;
use Hamcrest\Matcher;
use Hamcrest\Util;

/**
 * Matches if traversable size satisfies a nested matcher.
 */
class IsTraversableWithSize extends FeatureMatcher
{

    public function __construct(Matcher $sizeMatcher)
    {}

    protected function featureValueOf($actual)
    {}

    /**
     * Does traversable size satisfy a given matcher?
     *
     * @factory
     * @param mixed $size
     */
    public static function traversableWithSize($size): self
    {}
}
