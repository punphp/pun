<?php

class ParentNarrowerDoc
{
    /** @var string[] */
    public array $items = [];
}

class SomeClass extends ParentNarrowerDoc
{
    public array $items = [1];
}
