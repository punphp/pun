<?php

namespace Probe;

function p0(string $v)
{
    return preg_replace_callback('/(\d+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p1(string $v)
{
    return preg_replace_callback('/x(\d+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p2(string $v)
{
    return preg_replace_callback('/x([0-9]+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p3(string $v)
{
    return preg_replace_callback('/(a|b)/', function ($m) {
        return $m[0];
    }, $v);
}

function p4(string $v)
{
    return preg_replace_callback('/x(a|b)/', function ($m) {
        return $m[0];
    }, $v);
}

function p5(string $v)
{
    return preg_replace_callback('/(foo)/', function ($m) {
        return $m[0];
    }, $v);
}

function p6(string $v)
{
    return preg_replace_callback('/x(\d)?/', function ($m) {
        return $m[0];
    }, $v);
}

function p7(string $v)
{
    return preg_replace_callback('/x(?:\d+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p8(string $v)
{
    return preg_replace_callback('/(?<id>\d+)x/', function ($m) {
        return $m[0];
    }, $v);
}

function p9(string $v)
{
    return preg_replace_callback('/x(\w+)(\d+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p10(string $v)
{
    return preg_replace_callback('/x(\d+)/i', function ($m) {
        return $m[0];
    }, $v);
}

function p11(string $v)
{
    return preg_replace_callback('/x(-?\d+)/', function ($m) {
        return $m[0];
    }, $v);
}
-----
<?php

namespace Probe;

function p0(string $v)
{
    return preg_replace_callback('/(\d+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p1(string $v)
{
    return preg_replace_callback('/x(\d+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p2(string $v)
{
    return preg_replace_callback('/x([0-9]+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p3(string $v)
{
    return preg_replace_callback('/(a|b)/', function (array $m) {
        return $m[0];
    }, $v);
}

function p4(string $v)
{
    return preg_replace_callback('/x(a|b)/', function (array $m) {
        return $m[0];
    }, $v);
}

function p5(string $v)
{
    return preg_replace_callback('/(foo)/', function (array $m) {
        return $m[0];
    }, $v);
}

function p6(string $v)
{
    return preg_replace_callback('/x(\d)?/', function ($m) {
        return $m[0];
    }, $v);
}

function p7(string $v)
{
    return preg_replace_callback('/x(?:\d+)/', function (array $m) {
        return $m[0];
    }, $v);
}

function p8(string $v)
{
    return preg_replace_callback('/(?<id>\d+)x/', function ($m) {
        return $m[0];
    }, $v);
}

function p9(string $v)
{
    return preg_replace_callback('/x(\w+)(\d+)/', function ($m) {
        return $m[0];
    }, $v);
}

function p10(string $v)
{
    return preg_replace_callback('/x(\d+)/i', function ($m) {
        return $m[0];
    }, $v);
}

function p11(string $v)
{
    return preg_replace_callback('/x(-?\d+)/', function (array $m) {
        return $m[0];
    }, $v);
}
