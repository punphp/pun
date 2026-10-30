<?php

namespace App;

use App\Support\Browser;
use Behat\Behat\Context\Context;
use DateTimeImmutable as Moment;

final class Item
{
}

abstract class BaseContext implements Context
{
}

final class FeatureContext extends BaseContext
{
    /**
     * @var Browser
     */
    public static $browser;
    /** @var Item */
    private static $item = null;
    /** @var Moment */
    public static $moment;
    /** @var \DateTime The time */
    protected static $time;
    /** @var Missing */
    public static $missing;
    /** @var Collection<int, Item> */
    public static $collection;
    /** @var Item|null */
    public static $union;
    /** @var int */
    public static $count;
    /** @var self */
    public static $self;
    /** @var static */
    public static $static;
    /** @var parent */
    public static $parent;
    /** @var object */
    public static $object;
    /** @var Item */
    public static $withDefault = 'x';
    /** @var Item */
    public $notStatic;
    /** @var Collection<int, Item>[] */
    public static $listOfGenerics;
    /** @var Item|Collection<int, Item> */
    public static $unionGeneric;
    /** @var Collection<int, Item */
    public static $unclosed;
    /**
     * @var Collection<int,
     *     Item>
     */
    public static $multiLine;
    /** @var Collection<int, array<int, Item>> */
    public static $nested;
    /**
     * @var Item
     *     The item, over two lines
     */
    public static $multiLineName;
    /**
     * @var ?Item
     *     The item, over two lines
     */
    public static $multiLineNullable;
    /**
     * @var Item|null
     *     The item, over two lines
     */
    public static $multiLineUnion;
    public static $noDoc;
}

final class NotAContext extends Item
{
    /** @var Item */
    public static $item;
}
-----
<?php

namespace App;

use App\Support\Browser;
use Behat\Behat\Context\Context;
use DateTimeImmutable as Moment;

final class Item
{
}

abstract class BaseContext implements Context
{
}

final class FeatureContext extends BaseContext
{
    public static ?\App\Support\Browser $browser = null;
    private static ?\App\Item $item = null;
    public static ?\Moment $moment = null;
    /** @var \DateTime The time */
    protected static ?\DateTime $time = null;
    public static ?\App\Missing $missing = null;
    /** @var Collection<int, Item> */
    public static ?\App\Collection $collection = null;
    /** @var Item|null */
    public static $union;
    /** @var int */
    public static $count;
    public static ?\App\FeatureContext $self = null;
    /** @var static */
    public static $static;
    /** @var parent */
    public static $parent;
    /** @var object */
    public static $object;
    /** @var Item */
    public static $withDefault = 'x';
    /** @var Item */
    public $notStatic;
    /** @var Collection<int, Item>[] */
    public static $listOfGenerics;
    /** @var Item|Collection<int, Item> */
    public static $unionGeneric;
    /** @var Collection<int, Item */
    public static $unclosed;
    /**
     * @var Collection<int,
     *     Item>
     */
    public static ?\App\Collection $multiLine = null;
    /** @var Collection<int, array<int, Item>> */
    public static ?\App\Collection $nested = null;
    /**
     * @var Item
     *     The item, over two lines
     */
    public static ?\App\Item $multiLineName = null;
    /**
     * @var ?Item
     *     The item, over two lines
     */
    public static $multiLineNullable;
    /**
     * @var Item|null
     *     The item, over two lines
     */
    public static $multiLineUnion;
    public static $noDoc;
}

final class NotAContext extends Item
{
    /** @var Item */
    public static $item;
}
