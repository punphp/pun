<?php

namespace ConstantReceiverVersions;

function versions(object $object, string $string, int $int): void {
    echo $object::class;
    echo $string::class;
    echo $int::class;
    echo 'stdClass'::class;
    if (PHP_VERSION_ID >= 80000) {
        echo $object::class;
        echo $string::class;
    }
    if (PHP_VERSION_ID < 80000) {
        echo $object::class;
    }
}
