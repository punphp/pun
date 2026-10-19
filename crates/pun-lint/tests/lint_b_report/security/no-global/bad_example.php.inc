<?php

function foo(): void {
    global $bar;
    // ...
}
-----
4:5: Unsafe use of `global` keyword.
