<?php

namespace Probe;

function a(string $v)
{
    return preg_replace_callback('/(\w+)/', function ($m) {
        return $m[1];
    }, $v);
}

function b(string $v)
{
    return preg_replace_callback('/a(b)c/', function ($m) {
        return $m[1];
    }, $v);
}

function c(string $v)
{
    return preg_replace_callback('/(a)|b/', function ($m) {
        return $m[0];
    }, $v);
}

function d(string $v, string $pattern)
{
    return preg_replace_callback($pattern, function ($m) {
        return $m[0];
    }, $v);
}

function e(string $v)
{
    return preg_replace_callback('/[\x80-\xFF][\x80-\xBF]*|\x1A/', function ($m) {
        return $m[0];
    }, $v);
}

function f(string $v)
{
    return preg_replace_callback('/x/', function ($m) {
        return $m[0];
    }, $v);
}

function g(string $v)
{
    return preg_replace_callback(['/x/', '/y/'], function ($m) {
        return $m[0];
    }, $v);
}
-----
<?php

namespace Probe;

function a(string $v)
{
    return preg_replace_callback('/(\w+)/', function (array $m) {
        return $m[1];
    }, $v);
}

function b(string $v)
{
    return preg_replace_callback('/a(b)c/', function (array $m) {
        return $m[1];
    }, $v);
}

function c(string $v)
{
    return preg_replace_callback('/(a)|b/', function (array $m) {
        return $m[0];
    }, $v);
}

function d(string $v, string $pattern)
{
    return preg_replace_callback($pattern, function (array $m) {
        return $m[0];
    }, $v);
}

function e(string $v)
{
    return preg_replace_callback('/[\x80-\xFF][\x80-\xBF]*|\x1A/', function (array $m) {
        return $m[0];
    }, $v);
}

function f(string $v)
{
    return preg_replace_callback('/x/', function (array $m) {
        return $m[0];
    }, $v);
}

function g(string $v)
{
    return preg_replace_callback(['/x/', '/y/'], function (array $m) {
        return $m[0];
    }, $v);
}
