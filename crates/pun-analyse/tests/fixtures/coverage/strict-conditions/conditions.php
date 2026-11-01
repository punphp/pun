<?php declare(strict_types = 1);

namespace StrictConditions;

class Conditions
{
    /** @var bool */
    public $documented = true;

    public function ifs(bool $bool, ?bool $nullable, int $int, string $string, $implicit, mixed $explicit, array $list): void
    {
        if ($bool) {
        }
        if ($nullable) {
        } elseif ($int) {
        } elseif ($bool) {
        } else if ($string) {
        }
        if ($implicit) {
        }
        if ($explicit) {
        }
        if ($list):
        elseif ($this):
        endif;
        if ($int > 1 && $bool) {
        }
        if (!$string) {
        }
    }

    public function ternaries(bool $bool, int $int, ?string $string): string
    {
        $a = $bool ? 'a' : 'b';
        $b = $int ? 'a' : 'b';
        $c = $string ?: 'b';
        $d = $string ? $string : 'b';
        $e = $this ? 'a' : 'b';
        return $a . $b . $c . $d . $e;
    }

    public function loops(int $int, bool $bool, ?\stdClass $object, array $rows): void
    {
        while ($int) {
            $int--;
        }
        while ($bool) {
            $bool = false;
        }
        while ($row = array_pop($rows)) {
        }
        do {
            $int--;
        } while ($int);
        do {
        } while ($bool);
        while ($this) {
            break;
        }
        while ($int):
            $int = 0;
        endwhile;
    }
}

function outside(int $int, bool $bool): void
{
    if ($int) {
    }
    if ($bool) {
    }
    echo $int ? 'a' : 'b';
}
