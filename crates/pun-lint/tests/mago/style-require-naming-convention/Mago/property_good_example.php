<?php

final class Foo {
    public string $myProperty;

    public function __construct(
        public int $myPromotedProperty,
    ) {}
}
