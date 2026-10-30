<?php

$checker = new class {
    public function check(object $o)
    {
        return is_a($o, self::class) || is_a($o, static::class);
    }
};
