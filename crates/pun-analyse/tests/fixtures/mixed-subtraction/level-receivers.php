<?php

namespace MixedLevelProbe;

function explicit(mixed $value): void {
    if ($value !== null) {
        echo $value::EXPLICIT;
        $value->run();
        echo $value->property;
    }
}

function implicit($value): void {
    if ($value !== null) {
        echo $value::IMPLICIT;
        $value->run();
        echo $value->property;
    }
}

/**
 * @template T
 * @param T $value
 */
function template($value): void {
    if ($value !== null) {
        echo $value::TEMPLATE;
    }
}

/**
 * @template T of mixed
 * @param T $value
 */
function bounded($value): void {
    if ($value !== null) {
        echo $value::BOUNDED;
    }
}

function concrete(?int $value): void {
    if ($value !== null) {
        echo $value::CONCRETE;
        $value->run();
        echo $value->property;
    }
}
