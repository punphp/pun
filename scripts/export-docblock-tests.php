<?php

// Export PHP-CS-Fixer's tests/DocBlock/*Test.php data providers to JSON for pun-fixers' DocBlock unit tests.
//
//   php scripts/export-docblock-tests.php > crates/pun-fixers/src/analyzers/docblock/tests.json

$root = dirname(__DIR__).'/.oracle/php-cs-fixer';
require $root.'/vendor/autoload.php';

use PHPUnit\Framework\Attributes\DataProvider;

function dump_value(mixed $v): mixed
{
    if (is_array($v)) {
        return ['__list' => array_is_list($v), 'items' => array_map('dump_value', $v)];
    }
    if (is_object($v)) {
        return ['__class' => get_class($v)] + array_map('dump_value', (fn () => get_object_vars($this))->call($v));
    }

    return $v;
}

$out = [];
foreach (glob($root.'/tests/DocBlock/*Test.php') as $file) {
    $class = 'PhpCsFixer\\Tests\\DocBlock\\'.basename($file, '.php');
    $rc = new ReflectionClass($class);
    $out[basename($file, '.php').'::__static'] = array_map('dump_value', $rc->getConstants() + $rc->getStaticProperties());
    foreach ($rc->getMethods() as $m) {
        foreach ($m->getAttributes(DataProvider::class) as $a) {
            $provider = $a->getArguments()[0];
            foreach ($rc->getMethod($provider)->invoke(null) as $key => $args) {
                $out[basename($file, '.php').'::'.$m->getName()][] = ['key' => $key, 'args' => array_map('dump_value', $args)];
            }
        }
    }
}
echo json_encode($out, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES | JSON_UNESCAPED_UNICODE | JSON_THROW_ON_ERROR)."\n";
