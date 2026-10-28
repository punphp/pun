<?php

final class SomeClass
{
    
    public function run(string $contents): array
    {
        return \Foo\json_decode($contents, true);
    }
}
