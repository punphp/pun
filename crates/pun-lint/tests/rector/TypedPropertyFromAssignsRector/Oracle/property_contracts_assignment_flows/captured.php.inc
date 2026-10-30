<?php

namespace QualityPropertyAssignmentFlows;

final class WriteTargets
{
    private $peer;
    private static $named;
    private static $unknown;
    private $ratio = 0.5;
    private $positive;
    private $nonempty;
    private $nested = [];
    private $empty = [];
    private $words;

    public function write(self $other, object $foreign, string $class, float $ratio, string $text, int $number): void
    {
        $other->peer = 1;
        $foreign->peer = 'foreign';
        WriteTargets::$named = 'local';
        $class::$unknown = 'unknown';
        $this->ratio = $ratio;
        $this->nested = [[1, 2], ['key' => 3]];
        $this->empty = [];
        $this->words = explode(',', $text);
        if ($number > 0) {
            $this->positive = $number;
        }
        if ($text !== '') {
            $this->nonempty = $text;
        }
    }
}

final class ForeignDynamicWrite
{
    private $value;

    public function __construct(object $other, string $name)
    {
        $other->{$name} = 2;
        $this->value = 1;
    }
}

final class LocalDynamicWrite
{
    private static $value;

    public function __construct(string $name)
    {
        static::${$name} = 2;
        static::$value = 1;
    }
}

final class NotPlainAssignments
{
    private $value = 0;
    private $alias = 0;

    public function write(int $value): void
    {
        $this->value += $value;
        $this->alias = &$value;
    }
}
-----
<?php

namespace QualityPropertyAssignmentFlows;

final class WriteTargets
{
    private ?int $peer = null;
    private static ?string $named = null;
    private static $unknown;
    private float $ratio = 0.5;
    private ?int $positive = null;
    private ?string $nonempty = null;
    private array $nested = [];
    private array $empty = [];
    private ?array $words = null;

    public function write(self $other, object $foreign, string $class, float $ratio, string $text, int $number): void
    {
        $other->peer = 1;
        $foreign->peer = 'foreign';
        WriteTargets::$named = 'local';
        $class::$unknown = 'unknown';
        $this->ratio = $ratio;
        $this->nested = [[1, 2], ['key' => 3]];
        $this->empty = [];
        $this->words = explode(',', $text);
        if ($number > 0) {
            $this->positive = $number;
        }
        if ($text !== '') {
            $this->nonempty = $text;
        }
    }
}

final class ForeignDynamicWrite
{
    private int $value;

    public function __construct(object $other, string $name)
    {
        $other->{$name} = 2;
        $this->value = 1;
    }
}

final class LocalDynamicWrite
{
    private static $value;

    public function __construct(string $name)
    {
        static::${$name} = 2;
        static::$value = 1;
    }
}

final class NotPlainAssignments
{
    private int $value = 0;
    private int $alias = 0;

    public function write(int $value): void
    {
        $this->value += $value;
        $this->alias = &$value;
    }
}
