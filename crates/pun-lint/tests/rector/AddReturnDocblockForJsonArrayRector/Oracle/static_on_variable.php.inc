<?php

final class SomeClass
{
    
    public function run(string $contents): array
    {
        $class = 'Nette\\Utils\\Json';

        return $class::decode($contents, true);
    }
}
