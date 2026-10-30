<?php

final class GlobalService {}

final class GlobalContainerTest extends \PHPUnit\Framework\TestCase
{
    /** @var GlobalService */
    private $service;

    protected function setUp(): void
    {
        $this->service = $this->get('a');
    }
}
-----
<?php

final class GlobalService {}

final class GlobalContainerTest extends \PHPUnit\Framework\TestCase
{
    private \GlobalService $service;

    protected function setUp(): void
    {
        $this->service = $this->get('a');
    }
}
