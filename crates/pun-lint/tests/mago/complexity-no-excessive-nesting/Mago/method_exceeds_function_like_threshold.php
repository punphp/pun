<?php

class Foo {
    public function bar(): void {
        if ($a) { if ($b) { if ($c) {
            echo "deep in method";
        } } }
    }
}
