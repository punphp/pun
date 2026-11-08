<?php

class ParentDummyDoc
{
    /** @var array */
    public array $items = [];
}

class SomeClass extends ParentDummyDoc
{
    public array $items = [1];
}
