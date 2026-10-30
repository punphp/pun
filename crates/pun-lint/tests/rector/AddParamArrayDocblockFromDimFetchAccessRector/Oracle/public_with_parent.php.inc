<?php

class ParentClass
{
    public function process(array $data): void
    {
    }
}

final class PublicWithParent extends ParentClass
{
    public function process(array $data): void
    {
        echo $data['a'];
    }
}
