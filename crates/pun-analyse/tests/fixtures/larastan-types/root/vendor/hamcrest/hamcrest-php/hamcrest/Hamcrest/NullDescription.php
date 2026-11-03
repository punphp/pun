<?php
namespace Hamcrest;

/*
 Copyright (c) 2009 hamcrest.org
 */

/**
 * Null implementation of {@link Hamcrest\Description}.
 */
class NullDescription implements Description
{

    public function appendText(string $text): self
    {}

    public function appendDescriptionOf(SelfDescribing $value): self
    {}

    public function appendValue($value): self
    {}

    public function appendValueList(string $start, string $separator, string $end, $values): self
    {}

    public function appendList(string $start, string $separator, string $end, $values): self
    {}

    public function __toString()
    {}
}
