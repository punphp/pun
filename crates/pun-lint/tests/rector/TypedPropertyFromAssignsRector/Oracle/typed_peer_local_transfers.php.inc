<?php

namespace QualityPropertyContracts;

final class LocalTransfers
{
    private $ratio = 0.5;
    private $positive;
    private $nonempty;
    private $nested = [];
    private $empty = [];
    private $words;
    private $peer;
    private static $named;
    private static $unrelated;
    /** @var null */
    private $nullDoc;
    /** @var float */
    private $documentedFloat;
    /** @var bool */
    private $documentedBool;

    public function __construct(float $ratio, self $other, object $foreign, string $text, int $number, string $class)
    {
        $this->ratio = $ratio;
        $this->nested = [[1, 2], ['key' => 3]];
        $this->empty = [];
        $this->words = explode(',', $text);
        $other->peer = 1;
        $foreign->peer = 'foreign';
        LocalTransfers::$named = 'local';
        $class::$unrelated = 'unknown';
        $this->nullDoc = 1;
        $this->documentedFloat = $ratio;
        $this->documentedBool = true;
        if ($number > 0) {
            $this->positive = $number;
        }
        if ($text !== '') {
            $this->nonempty = $text;
        }
    }
}

final class DynamicOther
{
    private $value;

    public function __construct(object $other, string $name)
    {
        $other->{$name} = 2;
        $this->value = 1;
    }
}

final class DynamicStatic
{
    private static $value;

    public function __construct(string $name)
    {
        self::${$name} = 2;
        self::$value = 1;
    }
}

final class OperatorsAndReferences
{
    private $value = 0;
    private $alias = 0;

    public function change(int $value): void
    {
        $this->value += $value;
        $this->alias = &$value;
    }
}
-----
<?php

namespace QualityPropertyContracts;

final class LocalTransfers
{
    private float $ratio = 0.5;
    private ?int $positive = null;
    private ?string $nonempty = null;
    private array $nested = [];
    private array $empty = [];
    private array $words;
    private int $peer;
    private static string $named;
    private static $unrelated;
    /** @var null */
    private $nullDoc;
    private float $documentedFloat;
    private bool $documentedBool;

    public function __construct(float $ratio, self $other, object $foreign, string $text, int $number, string $class)
    {
        $this->ratio = $ratio;
        $this->nested = [[1, 2], ['key' => 3]];
        $this->empty = [];
        $this->words = explode(',', $text);
        $other->peer = 1;
        $foreign->peer = 'foreign';
        LocalTransfers::$named = 'local';
        $class::$unrelated = 'unknown';
        $this->nullDoc = 1;
        $this->documentedFloat = $ratio;
        $this->documentedBool = true;
        if ($number > 0) {
            $this->positive = $number;
        }
        if ($text !== '') {
            $this->nonempty = $text;
        }
    }
}

final class DynamicOther
{
    private int $value;

    public function __construct(object $other, string $name)
    {
        $other->{$name} = 2;
        $this->value = 1;
    }
}

final class DynamicStatic
{
    private static $value;

    public function __construct(string $name)
    {
        self::${$name} = 2;
        self::$value = 1;
    }
}

final class OperatorsAndReferences
{
    private int $value = 0;
    private int $alias = 0;

    public function change(int $value): void
    {
        $this->value += $value;
        $this->alias = &$value;
    }
}
