<?php

$a = new class extends \Unknown\Base {
    public function fail()
    {
        throw new \RuntimeException();
    }
};
