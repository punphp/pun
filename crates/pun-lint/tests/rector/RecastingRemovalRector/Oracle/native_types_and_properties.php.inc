<?php

namespace App;

enum Suit: string
{
    case Hearts = 'h';
}

class Box
{
    public int $count = 0;
    public $loose;
    public ?string $label = null;
    public static int $total = 0;
    public array $items = [];

    public function run(string $s, int $i, float $f, bool $b, array $a, object $o, ?int $n, int|string $u, \stdClass $std, $untyped, iterable $it, callable $c)
    {
        $x = (string) $s;
        $x = (int) $i;
        $x = (integer) $i;
        $x = (float) $f;
        $x = (double) $f;
        $x = (bool) $b;
        $x = (boolean) $b;
        $x = (array) $a;
        $x = (object) $o;
        $x = (object) $std;
        $x = (int) $n;
        $x = (string) $u;
        $x = (string) $untyped;
        $x = (array) $it;
        $x = (int) $this->count;
        $x = (string) $this->loose;
        $x = (string) $this->label;
        $x = (int) self::$total;
        $x = (int) static::$total;
        $x = (array) $this->items;
        $x = (object) $this;
        $x = (object) Suit::Hearts;
        $x = (string) 'abc';
        $x = (int) 5;
        $x = (float) 1.5;
        $x = (bool) true;
        $x = (array) [1, 2];
        $x = (array) [];
        $x = (array) ['a' => 1];
        $x = (string) ($s . 'x');
        $x = (string) strtolower($s);
        $x = (string) trim($s);
        $x = (int) strlen($s);
        $x = (int) count($a);
        $x = (array) explode(',', $s);
        $x = (array) array_values($a);
        $x = (bool) ($i > 1);
        $x = (bool) !$b;
        $x = (string) $this->name();
        $x = (int) $a['k'];
        $x = (string) PHP_EOL;
        $x = (int) PHP_INT_MAX;
        $x = (string) Suit::Hearts->value;
        $x = (string) $std->prop;
        $x = (string) $o?->prop;
        $x = (string) sprintf('%s', $s);
        $x = (string) implode(',', $a);
        $x = (string) ucfirst($s);
        $x = (string) json_encode($a);
        $x = (int) intdiv($i, 2);
        $x = (int) abs($i);
        $x = (float) round($f);
        $x = (string) \Closure::class;
        $x = (string) self::class;
        $x = (string) $s[0];
        $list = [];
        $list[] = 1;
        $x = (array) $list;
        foreach ($a as $v) {
            $list[] = $v;
        }
        $x = (array) $list;
        if ($b) {
            $maybe = 1;
        }
        $x = (int) $maybe;
        $name = 'other';
        $other = 1;
        $x = (int) $other;
        return $x;
    }

    private function name(): string
    {
        return 'n';
    }
}
-----
<?php

namespace App;

enum Suit: string
{
    case Hearts = 'h';
}

class Box
{
    public int $count = 0;
    public $loose;
    public ?string $label = null;
    public static int $total = 0;
    public array $items = [];

    public function run(string $s, int $i, float $f, bool $b, array $a, object $o, ?int $n, int|string $u, \stdClass $std, $untyped, iterable $it, callable $c)
    {
        $x = $s;
        $x = $i;
        $x = $i;
        $x = $f;
        $x = $f;
        $x = $b;
        $x = $b;
        $x = $a;
        $x = (object) $o;
        $x = $std;
        $x = (int) $n;
        $x = (string) $u;
        $x = (string) $untyped;
        $x = (array) $it;
        $x = $this->count;
        $x = (string) $this->loose;
        $x = (string) $this->label;
        $x = self::$total;
        $x = static::$total;
        $x = $this->items;
        $x = (object) $this;
        $x = Suit::Hearts;
        $x = 'abc';
        $x = 5;
        $x = 1.5;
        $x = true;
        $x = [1, 2];
        $x = [];
        $x = ['a' => 1];
        $x = $s . 'x';
        $x = (string) strtolower($s);
        $x = trim($s);
        $x = strlen($s);
        $x = count($a);
        $x = explode(',', $s);
        $x = array_values($a);
        $x = $i > 1;
        $x = !$b;
        $x = $this->name();
        $x = (int) $a['k'];
        $x = (string) PHP_EOL;
        $x = (int) PHP_INT_MAX;
        $x = (string) Suit::Hearts->value;
        $x = (string) $std->prop;
        $x = (string) $o?->prop;
        $x = sprintf('%s', $s);
        $x = implode(',', $a);
        $x = ucfirst($s);
        $x = (string) json_encode($a);
        $x = intdiv($i, 2);
        $x = abs($i);
        $x = round($f);
        $x = \Closure::class;
        $x = self::class;
        $x = (string) $s[0];
        $list = [];
        $list[] = 1;
        $x = $list;
        foreach ($a as $v) {
            $list[] = $v;
        }
        $x = $list;
        if ($b) {
            $maybe = 1;
        }
        $x = (int) $maybe;
        $name = 'other';
        $other = 1;
        $x = (int) $other;
        return $x;
    }

    private function name(): string
    {
        return 'n';
    }
}
