<?php

function a()
{
    $file = new SplFileObject('x');
    $f = function () use ($file) {
        $file->fputcsv([1]);
    };
    $g = function () {
        $file->fputcsv([1]);
    };
    $h = fn () => $file->fputcsv([1]);
}

function b()
{
    $file = new SplFileObject('x');
    $file = &$other;
    $file->fputcsv([1]);
}

function c()
{
    $file = new SplFileObject('x');
    if ($x) {
        $file = other();
    }
    $file->fputcsv([1]);
}

function d()
{
    $file = new SplFileObject('x');
    $file .= 'y';
    $file->fputcsv([1]);
}

function e()
{
    $file = new SplFileObject('x');
    $file->fputcsv([1]);
    $file = new SplFileObject('y');
    $file->fputcsv([1]);
}

function g(SplFileObject ...$file)
{
    $file->fputcsv([1]);
}

function h(SplFileObject $file)
{
    [$file] = $x;
    $file->fputcsv([1]);
}

function i()
{
    $file = new SplFileObject('x');
    $f = function () use (&$file) {
        $file = other();
    };
    $f();
    $file->fputcsv([1]);
}

function j()
{
    $file = new SplFileObject('x');
    foreach ($x as $file) {
    }
    $file->fputcsv([1]);
}

function k()
{
    $file = new SplFileObject('x');
    static $file;
    $file->fputcsv([1]);
}

function l()
{
    $file = new SplFileObject('x');
    $file->fputcsv([1]);
    $file = $other;
}

function m()
{
    $file = new $class('x');
    $file->fputcsv([1]);
}

class N
{
    public function m(SplFileObject $file)
    {
        $file->fputcsv([1]);
    }
}
-----
<?php

function a()
{
    $file = new SplFileObject('x');
    $f = function () use ($file) {
        $file->fputcsv([1], escape: '\\');
    };
    $g = function () {
        $file->fputcsv([1]);
    };
    $h = fn () => $file->fputcsv([1], escape: '\\');
}

function b()
{
    $file = new SplFileObject('x');
    $file = &$other;
    $file->fputcsv([1]);
}

function c()
{
    $file = new SplFileObject('x');
    if ($x) {
        $file = other();
    }
    $file->fputcsv([1]);
}

function d()
{
    $file = new SplFileObject('x');
    $file .= 'y';
    $file->fputcsv([1]);
}

function e()
{
    $file = new SplFileObject('x');
    $file->fputcsv([1], escape: '\\');
    $file = new SplFileObject('y');
    $file->fputcsv([1], escape: '\\');
}

function g(SplFileObject ...$file)
{
    $file->fputcsv([1]);
}

function h(SplFileObject $file)
{
    [$file] = $x;
    $file->fputcsv([1]);
}

function i()
{
    $file = new SplFileObject('x');
    $f = function () use (&$file) {
        $file = other();
    };
    $f();
    $file->fputcsv([1]);
}

function j()
{
    $file = new SplFileObject('x');
    foreach ($x as $file) {
    }
    $file->fputcsv([1]);
}

function k()
{
    $file = new SplFileObject('x');
    static $file;
    $file->fputcsv([1]);
}

function l()
{
    $file = new SplFileObject('x');
    $file->fputcsv([1], escape: '\\');
    $file = $other;
}

function m()
{
    $file = new $class('x');
    $file->fputcsv([1]);
}

class N
{
    public function m(SplFileObject $file)
    {
        $file->fputcsv([1], escape: '\\');
    }
}
