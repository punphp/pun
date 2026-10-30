<?php

use Nette\Utils\Json;

final class SomeClass
{
    
    public function run(string $contents): array
    {
        return Json::encode($contents, true);
    }
}
