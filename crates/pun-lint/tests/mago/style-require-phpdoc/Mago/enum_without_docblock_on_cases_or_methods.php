<?php

enum Foo {
    case Bar;
    case Baz;

    public function toString(): string {
        return $this->name;
    }
}
