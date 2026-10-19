<?php

final class Holder
{
    private mixed $value;

    public function get()
    {
        return $this->value;
    }
}
