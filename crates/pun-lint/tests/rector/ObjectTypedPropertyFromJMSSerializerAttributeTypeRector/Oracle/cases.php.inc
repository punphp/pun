<?php

namespace App;

use JMS\Serializer\Annotation\Type;

final class Item {}

final class Objects
{
    #[Type(Item::class)]
    private $item;
    #[Type('App\Item')]
    private $written;
    #[Type('datetime')]
    private $lower;
    #[Type("DateTime<'Y-m-d'>")]
    private $formatted;
    #[Type('App\Missing')]
    private $missing;
    #[Type('array<App\Item>')]
    private $generic;
    #[Type('string')]
    private $scalar;
    #[Type('mixed')]
    private $mixed;
    #[Type(Item::class)]
    private $withDefault = null;
}
-----
<?php

namespace App;

use JMS\Serializer\Annotation\Type;

final class Item {}

final class Objects
{
    #[Type(Item::class)]
    private ?\App\Item $item = null;
    #[Type('App\Item')]
    private ?\App\Item $written = null;
    #[Type('datetime')]
    private ?\datetime $lower = null;
    #[Type("DateTime<'Y-m-d'>")]
    private ?\DateTime $formatted = null;
    #[Type('App\Missing')]
    private $missing;
    #[Type('array<App\Item>')]
    private $generic;
    #[Type('string')]
    private $scalar;
    #[Type('mixed')]
    private $mixed;
    #[Type(Item::class)]
    private $withDefault = null;
}
