<?php

final class SomeClass
{
    
    public function run(string $contents): string
    {
        return json_decode($contents, true);
    }
}
