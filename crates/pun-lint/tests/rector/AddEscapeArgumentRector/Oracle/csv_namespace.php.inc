<?php

namespace App\Csv;

use SplFileObject as Csv;
use Other\{Thing, SplFileObject as Spl};
use function Foo\bar;
use const Foo\BAZ;
use Mixed\{function f, Some};

function a(Csv $file)
{
    $file->fputcsv([1]);
}

function b(SplFileObject $file)
{
    $file->fputcsv([1]);
}

function c(\SplFileObject $file)
{
    $file->$method([1]);
    $file->fputcsv([1]);
}

function d()
{
    $file = new namespace\SplFileObject('x');
    $file->fputcsv([1]);
}

function e(Csv\Sub $file)
{
    $file->fputcsv([1]);
}

function g(Spl $file)
{
    $file->fputcsv([1]);
}
-----
<?php

namespace App\Csv;

use SplFileObject as Csv;
use Other\{Thing, SplFileObject as Spl};
use function Foo\bar;
use const Foo\BAZ;
use Mixed\{function f, Some};

function a(Csv $file)
{
    $file->fputcsv([1], escape: '\\');
}

function b(SplFileObject $file)
{
    $file->fputcsv([1]);
}

function c(\SplFileObject $file)
{
    $file->$method([1]);
    $file->fputcsv([1], escape: '\\');
}

function d()
{
    $file = new namespace\SplFileObject('x');
    $file->fputcsv([1]);
}

function e(Csv\Sub $file)
{
    $file->fputcsv([1]);
}

function g(Spl $file)
{
    $file->fputcsv([1]);
}
