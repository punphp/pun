<?php
namespace Hamcrest\Arrays;

/*
 Copyright (c) 2009 hamcrest.org
 */

use Hamcrest\Description;
use Hamcrest\Matcher;

class SeriesMatchingOnce
{

    /**
     * @var array<Matcher>
     */
    private array $_elementMatchers;
    /**
     * @var list<int|string>
     */
    private array $_keys;
    private Description $_mismatchDescription;
    /**
     * @var int|string|null
     */
    private $_nextMatchKey;

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

    public function isFinished(): bool
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

    /**
     * @param mixed $item
     */
    private function _describeMismatch(Matcher $matcher, $item): void
    {}
}
