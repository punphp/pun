<?php

interface Example {
    public static function doTheThing(): void;

    public static function getSomeValue(): int;
}

Example::doTheThing();

echo Example::getSomeValue();
