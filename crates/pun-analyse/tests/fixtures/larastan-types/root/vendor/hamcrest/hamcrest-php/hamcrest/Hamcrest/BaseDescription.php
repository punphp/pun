<?php
namespace Hamcrest;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Internal\SelfDescribingValue;

/**
 * A {@link Hamcrest\Description} that is stored as a string.
 */
abstract class BaseDescription implements Description
{

    public function appendText(string $text): self
    {}

    public function appendDescriptionOf(SelfDescribing $value)
    {}

    public function appendValue($value): self
    {}

    public function appendValueList(string $start, string $separator, string $end, iterable $values): self
    {}

    public function appendList(string $start, string $separator, string $end, iterable $values): self
    {}

    // -- Protected Methods

    /**
     * Append the String <var>$str</var> to the description.
     * @param mixed $str
     */
    abstract protected function append($str): void;

    // -- Private Methods

    private function _toPhpSyntax(string $value): void
    {}
}
