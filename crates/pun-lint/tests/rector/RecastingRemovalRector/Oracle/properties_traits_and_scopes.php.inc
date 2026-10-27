<?php

namespace App\Casts;

trait Shared
{
    private string $text = '';
    private static string $label = '';

    public function fromTrait(): array
    {
        return [(string) $this->text, (string) self::$label];
    }
}

enum Suit: string
{
    case Hearts = 'h';

    public function value(): string
    {
        return (string) $this->value;
    }
}

class Base
{
    protected static string $inherited = '';
}

class Holder extends Base
{
    public static string $name = '';
    public static $untyped;
    public string $title = '';

    public function properties(string $s, string $prop, Holder $other, $mixed): array
    {
        return [
            (string) strtolower($s),
            (string) $this->{$prop},
            (string) self::$$prop,
            (string) Holder::$name,
            (string) parent::$inherited,
            (string) $other::$name,
            (string) $mixed::$name,
            (string) Holder::$untyped,
        ];
    }

    /**
     * @param lowercase-string $lower
     * @param array{a: int} $shape
     * @param list<int> $list
     */
    public function documented(string $lower, array $shape, array $list): array
    {
        /** @var list<int> $constant */
        $constant = [1, 2];

        return [(string) $lower, (array) $shape, (array) $list, (array) $constant];
    }

    public function anonymous(): object
    {
        return new class {
            public string $inner = '';

            public function run(): string
            {
                return (string) $this->inner;
            }
        };
    }
}

$top = 'value';
echo (string) $top;
-----
<?php

namespace App\Casts;

trait Shared
{
    private string $text = '';
    private static string $label = '';

    public function fromTrait(): array
    {
        return [(string) $this->text, self::$label];
    }
}

enum Suit: string
{
    case Hearts = 'h';

    public function value(): string
    {
        return (string) $this->value;
    }
}

class Base
{
    protected static string $inherited = '';
}

class Holder extends Base
{
    public static string $name = '';
    public static $untyped;
    public string $title = '';

    public function properties(string $s, string $prop, Holder $other, $mixed): array
    {
        return [
            (string) strtolower($s),
            (string) $this->{$prop},
            (string) self::$$prop,
            Holder::$name,
            parent::$inherited,
            $other::$name,
            (string) $mixed::$name,
            (string) Holder::$untyped,
        ];
    }

    /**
     * @param lowercase-string $lower
     * @param array{a: int} $shape
     * @param list<int> $list
     */
    public function documented(string $lower, array $shape, array $list): array
    {
        /** @var list<int> $constant */
        $constant = [1, 2];

        return [$lower, $shape, $list, (array) $constant];
    }

    public function anonymous(): object
    {
        return new class {
            public string $inner = '';

            public function run(): string
            {
                return $this->inner;
            }
        };
    }
}

$top = 'value';
echo $top;
