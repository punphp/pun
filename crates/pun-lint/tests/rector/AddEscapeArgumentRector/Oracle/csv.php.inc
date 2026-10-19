<?php

fputcsv($fp, $fields);
fgetcsv($fp, 0, ',', '"', '\\');
\str_getcsv($s, ',',);
fgetcsv($fp, escape: '\\');
str_getcsv();
Foo\fputcsv($fp, $fields);

function f(SplFileObject $file)
{
    $file->setCsvControl(',');
    $file->fgetcsv(',', '"', '\\');
    $file->FPUTCSV([1]);
    $file->fputcsv([1], ',', '"', '\\');
}

function g()
{
    $f = new \SplFileObject('x');
    $f->fputcsv([1], ',');
}

function h($f)
{
    $f->fputcsv([1]);
}

function i(SplFileObject $file)
{
    $file = other();
    $file->fputcsv([1]);
}
-----
<?php

fputcsv($fp, $fields, escape: '\\');
fgetcsv($fp, 0, ',', '"', '\\');
\str_getcsv($s, ',', escape: '\\',);
fgetcsv($fp, escape: '\\');
str_getcsv(escape: '\\');
Foo\fputcsv($fp, $fields);

function f(SplFileObject $file)
{
    $file->setCsvControl(',', escape: '\\');
    $file->fgetcsv(',', '"', '\\');
    $file->FPUTCSV([1]);
    $file->fputcsv([1], ',', '"', '\\');
}

function g()
{
    $f = new \SplFileObject('x');
    $f->fputcsv([1], ',', escape: '\\');
}

function h($f)
{
    $f->fputcsv([1]);
}

function i(SplFileObject $file)
{
    $file = other();
    $file->fputcsv([1]);
}
