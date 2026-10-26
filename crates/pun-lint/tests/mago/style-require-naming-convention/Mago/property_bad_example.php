<?php

final class Foo {
    public string $My_Property;

    public function __construct(
        public int $My_Promoted_Property,
    ) {}
}
