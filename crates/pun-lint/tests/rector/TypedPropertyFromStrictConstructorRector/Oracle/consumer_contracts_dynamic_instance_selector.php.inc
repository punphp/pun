<?php

final class ConsumerDynamicInstanceSelector
{
    private $slot;

    public function __construct(\stdClass $peer, string $slot, int $value)
    {
        $peer->{$slot} = 'poison';
        $this->slot = $value;
    }
}

final class ConsumerFixedInstanceSelector
{
    private $slot;

    public function __construct(\stdClass $peer, int $value)
    {
        $peer->other = 'poison';
        $this->slot = $value;
    }
}
-----
<?php

final class ConsumerDynamicInstanceSelector
{
    private int $slot;

    public function __construct(\stdClass $peer, string $slot, int $value)
    {
        $peer->{$slot} = 'poison';
        $this->slot = $value;
    }
}

final class ConsumerFixedInstanceSelector
{
    private int $slot;

    public function __construct(\stdClass $peer, int $value)
    {
        $peer->other = 'poison';
        $this->slot = $value;
    }
}
