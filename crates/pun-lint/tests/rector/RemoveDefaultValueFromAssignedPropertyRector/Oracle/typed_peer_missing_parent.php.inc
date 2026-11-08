<?php

final class TypedPeerMissingParent
{
    private int $value = 0;

    public function __construct()
    {
        parent::initialize();
    }
}
