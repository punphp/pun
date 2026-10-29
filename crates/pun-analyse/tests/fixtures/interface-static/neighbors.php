<?php

interface StaticContract {
    public static function getSomeValue(int $value): int;
}

class StaticConcrete implements StaticContract {
    public static function getSomeValue(int $value): int { return $value; }
}

/** @param class-string<StaticContract> $class */
function readClassString(string $class): void {
    $class::getSomeValue(1);
    $class::missing();
}

StaticConcrete::getSomeValue(1);
StaticConcrete::missing();
StaticContract::getSomeValue('wrong');
StaticContract::missing();
