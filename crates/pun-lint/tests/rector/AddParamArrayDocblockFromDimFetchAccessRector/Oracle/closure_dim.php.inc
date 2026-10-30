<?php

final class ClosureDim
{
    private function process(array $data): void
    {
        $f = function () use ($data) {
            echo $data['a'];
        };
    }
}
