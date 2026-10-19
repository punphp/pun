<?php

if ($x) {
    foo();
}
if (PHP_VERSION_ID < $x) {
    foo();
}
if (PHP_VERSION_ID < 99999999999999999999) {
    foo();
}
if (PHP_VERSION_ID <= 70000) {
    foo();
}
if (PHP_VERSION_ID < 70000 + 1) {
    foo();
}
