<?php

final class ConsumerStaticPeer
{
    public static $other;
}

final class ConsumerDynamicStaticSelector
{
    private $slot;

    public function __construct(string $slot, int $value)
    {
        ConsumerStaticPeer::${$slot} = 'poison';
        $this->slot = $value;
    }
}

final class ConsumerFixedStaticSelector
{
    private $slot;

    public function __construct(int $value)
    {
        ConsumerStaticPeer::$other = 'poison';
        $this->slot = $value;
    }
}
-----
<?php

final class ConsumerStaticPeer
{
    public static $other;
}

final class ConsumerDynamicStaticSelector
{
    private int $slot;

    public function __construct(string $slot, int $value)
    {
        ConsumerStaticPeer::${$slot} = 'poison';
        $this->slot = $value;
    }
}

final class ConsumerFixedStaticSelector
{
    private int $slot;

    public function __construct(int $value)
    {
        ConsumerStaticPeer::$other = 'poison';
        $this->slot = $value;
    }
}
