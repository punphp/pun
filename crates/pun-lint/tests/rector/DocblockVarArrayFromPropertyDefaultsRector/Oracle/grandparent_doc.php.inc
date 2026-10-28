<?php

class GrandParentDoc
{
    /** @var int[] */
    public array $items = [];
}

class MiddleClass extends GrandParentDoc
{
}

class SomeClass extends MiddleClass
{
    public array $items = [1];
}
