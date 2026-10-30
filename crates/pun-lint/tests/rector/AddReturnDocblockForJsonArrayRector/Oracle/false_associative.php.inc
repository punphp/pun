<?php

final class SomeClass
{
    
    public function run(string $contents): array
    {
        return json_decode($contents, false);
    }
}
