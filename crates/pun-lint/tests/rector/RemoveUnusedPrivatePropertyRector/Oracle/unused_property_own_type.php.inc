<?php

final class Link
{
    private Link $next;

    private $p;

    public function f()
    {
        return $this->next->p;
    }
}
