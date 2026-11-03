<?php
namespace Hamcrest\Arrays;

/*
 Copyright (c) 2009 hamcrest.org
 */

use Hamcrest\Description;
use Hamcrest\Matcher;

class MatchingOnce
{

    /**
     * @var array<Matcher>
     */
    private array $_elementMatchers;
    private Description $_mismatchDescription;

    /**
     * @param array<Matcher> $elementMatchers
     */
    public function __construct(array $elementMatchers, Description $mismatchDescription)
    {}

    /**
     * @param mixed $item
     */
    public function matches($item): bool
    {}

    /**
     * @param mixed $items
     */
    public function isFinished($items): bool
    {}

    // -- Private Methods

    /**
     * @param mixed $item
     */
    private function _isNotSurplus($item): bool
    {}

    /**
     * @param mixed $item
     */
    private function _isMatched($item): bool
    {}
}
