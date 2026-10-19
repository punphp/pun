<?php

trait ClosureTrait
{
    public function f(\stdClass $o, array $a)
    {
        return [
            function () use ($o) {
                return ($o);
            },
            function () use ($o) {
                return $o?->name;
            },
            function () use ($o) {
                return $o?->name();
            },
            function () use ($a) {
                return $a[0];
            },
        ];
    }
}
