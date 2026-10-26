<?php

class Foo {
    public function bar(): void {
        echo 'inside method, not a top-level side-effect';
    }
}
