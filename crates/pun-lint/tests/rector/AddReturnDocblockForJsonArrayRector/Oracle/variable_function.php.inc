<?php

final class SomeClass
{
    
    public function run(string $contents): array
    {
        $fn = 'json_decode';

        return $fn($contents, true);
    }
}
