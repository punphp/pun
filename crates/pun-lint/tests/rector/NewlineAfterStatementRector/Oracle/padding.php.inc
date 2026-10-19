<?php

function a($x)
{
    if ($x) {
        echo 1;
    }
    echo 2;
    foreach ($x as $y) {
    }
    // comment after foreach
    echo 3;
    while ($x) {
    }

    echo 4;
    for (;;) {
    } echo 5;
    try {
    } catch (E $e) {
    }
    switch ($x) {
        case 1:
            if ($x) {
            }
            break;
    }
    do {
    } while ($x);
    if ($x) {
    }
    // trailing
}
class A {}
class B {}
interface I {}
trait T {}
function f() {}
$x = 1;
-----
<?php

function a($x)
{
    if ($x) {
        echo 1;
    }

    echo 2;
    foreach ($x as $y) {
    }

    // comment after foreach
    echo 3;
    while ($x) {
    }

    echo 4;
    for (;;) {
    } echo 5;
    try {
    } catch (E $e) {
    }

    switch ($x) {
        case 1:
            if ($x) {
            }

            break;
    }

    do {
    } while ($x);

    if ($x) {
    }

    // trailing
}

class A {}

class B {}

interface I {}

trait T {}

function f() {}

$x = 1;
