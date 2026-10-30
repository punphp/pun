<?php

final class MixedKeys
{
    private function process(array $data, $key): void
    {
        echo $data['a'] . $data[1] . $data[$key];
    }
}
