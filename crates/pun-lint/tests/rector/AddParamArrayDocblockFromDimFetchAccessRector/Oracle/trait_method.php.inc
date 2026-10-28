<?php

trait SomeTrait
{
    public function process(array $data): void
    {
        echo $data['a'];
    }
}
-----
<?php

trait SomeTrait
{
    /**
     * @param array<string, mixed> $data
     */
    public function process(array $data): void
    {
        echo $data['a'];
    }
}
