<?php

function f0($x, $sign, $v)
{
    if ($x) {
        foo();
    }
}

function f1($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0') < 1) {
        foo();
    }
}

function f3($x, $sign, $v)
{
    if (true === version_compare(PHP_VERSION, '7.0', '>=')) {
        foo();
    }
}

function f4($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', '>=') === true) {
        foo();
    }
}

function f5($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0') === 1) {
        foo();
    }
}

function f6($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0') === PHP_EOL) {
        foo();
    }
}

function f7($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0') !== 1) {
        foo();
    }
}

function f8($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0') != 1) {
        foo();
    }
}

function f9($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0') <> 1) {
        foo();
    }
}

function f10($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0') == 1) {
        foo();
    }
}

function f11($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0')) {
        foo();
    }
}

function f13($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', '==')) {
        foo();
    }
}

function f14($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', 'eq')) {
        foo();
    }
}

function f15($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', '!=')) {
        foo();
    }
}

function f16($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', 'ne')) {
        foo();
    }
}

function f17($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', '<>')) {
        foo();
    }
}

function f19($x, $sign, $v)
{
    if (version_compare(version1: PHP_VERSION, version2: '7.0', operator: '>=')) {
        foo();
    }
}

function f20($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, 70000, '>=')) {
        foo();
    }
}

function f21($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, 7.0, '>=')) {
        foo();
    }
}

function f22($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, $v, '>=')) {
        foo();
    }
}

function f23($x, $sign, $v)
{
    if (version_compare(PHP_VERSION_ID, '7.0', '>=')) {
        foo();
    }
}

function f24($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0', 'gt')) {
        foo();
    }
}
-----
<?php

function f0($x, $sign, $v)
{
    if ($x) {
        foo();
    }
}

function f1($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, '7.0') < 1) {
        foo();
    }
}

function f3($x, $sign, $v)
{
    foo();
}

function f4($x, $sign, $v)
{
    foo();
}

function f5($x, $sign, $v)
{
    foo();
}

function f6($x, $sign, $v)
{
}

function f7($x, $sign, $v)
{
}

function f8($x, $sign, $v)
{
}

function f9($x, $sign, $v)
{
}

function f10($x, $sign, $v)
{
    foo();
}

function f11($x, $sign, $v)
{
    foo();
}

function f13($x, $sign, $v)
{
}

function f14($x, $sign, $v)
{
}

function f15($x, $sign, $v)
{
    foo();
}

function f16($x, $sign, $v)
{
    foo();
}

function f17($x, $sign, $v)
{
    foo();
}

function f19($x, $sign, $v)
{
    foo();
}

function f20($x, $sign, $v)
{
    foo();
}

function f21($x, $sign, $v)
{
    foo();
}

function f22($x, $sign, $v)
{
    if (version_compare(PHP_VERSION, $v, '>=')) {
        foo();
    }
}

function f23($x, $sign, $v)
{
    foo();
}

function f24($x, $sign, $v)
{
    foo();
}
