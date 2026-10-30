<?php

final class SomeClass
{
    
    public function run(string $contents): array
    {
        if ($contents === '') {
            return [];
        }

        return json_decode($contents, true);
    }
}
