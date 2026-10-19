<?php

function f() {
    foreach ($items as $item) {
        try {
            a();
        } finally {
            if ($x) {
                return 1;
            } elseif ($y) {
                throw new E();
            } else {
                continue;
            }
            while (true) {
                break;
            }
            $f = function () {
                return 2;
            };
            break 2;
        }
    }
    try {
    } catch (E $e) {
        return 3;
    }
}
