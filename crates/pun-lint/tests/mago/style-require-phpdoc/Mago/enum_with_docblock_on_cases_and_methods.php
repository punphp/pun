<?php

enum Foo {
    /**
     * Bar case docs.
     */
    case Bar;

    /**
     * Baz case docs.
     */
    case Baz;

    /**
     * Returns the string representation.
     */
    public function toString(): string {
        return $this->name;
    }
}
