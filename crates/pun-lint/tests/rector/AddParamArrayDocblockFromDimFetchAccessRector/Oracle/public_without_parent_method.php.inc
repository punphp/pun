<?php

class ParentClass
{
}

class PublicWithoutParentMethod extends ParentClass
{
    public function process(array $data): void
    {
        echo $data['a'];
    }
}
-----
<?php

class ParentClass
{
}

class PublicWithoutParentMethod extends ParentClass
{
    /**
     * @param array<string, mixed> $data
     */
    public function process(array $data): void
    {
        echo $data['a'];
    }
}
