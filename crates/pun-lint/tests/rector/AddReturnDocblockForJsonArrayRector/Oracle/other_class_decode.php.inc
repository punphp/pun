<?php

final class SomeClass
{
    
    public function run(string $contents): array
    {
        return Other::decode($contents, true);
    }
}
