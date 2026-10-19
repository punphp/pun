<?php

$a = new class implements \Unknown\Contract {
    public function fail()
    {
        throw new \RuntimeException();
    }
};
