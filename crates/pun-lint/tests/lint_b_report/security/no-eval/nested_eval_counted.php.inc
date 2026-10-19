<?php

function dangerous() {
    eval('$x = 1;');

    if (true) {
        eval('$y = 2;');
    }
}
-----
4:5: Unsafe use of `eval` construct.
7:9: Unsafe use of `eval` construct.
