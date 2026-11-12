<?php

namespace App;

use function Other\dirname as other_dirname;

function dirnameDirConcat($b)
{
    $p = dirname(__DIR__) . "/a$b";
    $p = dirname(__DIR__) . "/a\tb";
    $p = dirname(__DIR__) . "/a";
    $p = dirname(__DIR__) . '//a/b';
    $p = dirname(__DIR__) . '/';
    $p = dirname(__DIR__) . '\\';
    $p = dirname(__DIR__) . '\\a\'b';
    $p = dirname(__DIR__) . 'a/b\\c';
    $p = \DIRNAME(__dir__) . '/a' . '/b';
    $p = 'x' . dirname(__DIR__) . '/a';
    $p = dirname(path: __DIR__) . '/a';
    $p = (dirname(__DIR__)) . ('/a');
    $p = dirname(__DIR__)   .   '/a';
    $p = dirname(__DIR__ . '/x') . '/a';
    $p = dirname(...) . '/a';
    $p = dirname(__DIR__,) . '/a';
    $p = dirname(...[__DIR__]) . '/a';
    $p = dirname(__DIR__) /* c */ . /* d */ '/a';
    $p = Foo\dirname(__DIR__) . '/a';
    $p = other_dirname(__DIR__) . '/a';
    $p = dirname(__DIR__) . b'/a';
    $p = dirname(__DIR__) . "/a\"b's";
    $p = dirname(__DIR__) . "/a\$b";
    $p = dirname(__FILE__) . '/a';
    $p = dirname(__DIR__) . $b;
    $p = dirname(__DIR__) + '/a';
    $p = '/a' . dirname(__DIR__);
    $p = dirname() . '/a';
    $p = $b(__DIR__) . '/a';
}
-----
<?php

namespace App;

use function Other\dirname as other_dirname;

function dirnameDirConcat($b)
{
    $p = dirname(__DIR__) . "/a$b";
    $p = __DIR__ . "/../a\tb";
    $p = __DIR__ . "/../a";
    $p = __DIR__ . '/../a/b';
    $p = __DIR__ . '/../';
    $p = __DIR__ . '\..\\';
    $p = __DIR__ . '\..\a\'b';
    $p = __DIR__ . '/../a/b\c';
    $p = __DIR__ . '/../a' . '/b';
    $p = 'x' . dirname(__DIR__) . '/a';
    $p = __DIR__ . '/../a';
    $p = (__DIR__) . ('/../a');
    $p = __DIR__   .   '/../a';
    $p = dirname(__DIR__ . '/x') . '/a';
    $p = dirname(...) . '/a';
    $p = __DIR__ . '/../a';
    $p = dirname(...[__DIR__]) . '/a';
    $p = __DIR__ /* c */ . /* d */ '/../a';
    $p = Foo\dirname(__DIR__) . '/a';
    $p = other_dirname(__DIR__) . '/a';
    $p = __DIR__ . '/../a';
    $p = __DIR__ . "/../a\"b's";
    $p = __DIR__ . "/../a\$b";
    $p = dirname(__FILE__) . '/a';
    $p = dirname(__DIR__) . $b;
    $p = dirname(__DIR__) + '/a';
    $p = '/a' . dirname(__DIR__);
    $p = dirname() . '/a';
    $p = $b(__DIR__) . '/a';
}
