<?php

class ParentWithoutDoc
{
    public array $items = [];
}

class SomeClass extends ParentWithoutDoc
{
    public array $items = [1];
}
