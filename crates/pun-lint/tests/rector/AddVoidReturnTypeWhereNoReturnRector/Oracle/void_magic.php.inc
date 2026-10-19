<?php

class VoidMagic
{
    public function __destruct()
    {
        echo 1;
    }
}
