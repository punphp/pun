<?php

function f(int $n, bool $flag): void {
    if ($n > 0) {
        echo 'a';
    } elseif ($flag) {
        echo 'b';
    }
}
