<?php

namespace App;

class RedundantOracle
{
    private ?string $prop = null;

    public function ors(?string $s, ?int $i, ?float $f, ?bool $b, ?array $a, ?object $o, ?self $me, ?\Countable $c, string|int|null $u, $mixed, string $plain, null|false|string $fs)
    {
        if ($s === null || !is_string($s)) {}
        if (null === $s || !is_string($s)) {}
        if ($s === null || ! \is_string($s)) {}
        if ($s === null || !IS_STRING($s)) {}
        if ($s === NULL || !is_string($s)) {}
        if ($s == null || !is_string($s)) {}
        if ($s !== null || !is_string($s)) {}
        if ($s === null || is_string($s)) {}
        if ($s === null || !is_int($s)) {}
        if ($s === null or !is_string($s)) {}
        if ($i === null || !is_int($i)) {}
        if ($i === null || !is_integer($i)) {}
        if ($i === null || !is_long($i)) {}
        if ($i === null || !is_numeric($i)) {}
        if ($f === null || !is_float($f)) {}
        if ($f === null || !is_double($f)) {}
        if ($b === null || !is_bool($b)) {}
        if ($a === null || !is_array($a)) {}
        if ($a === null || !is_iterable($a)) {}
        if ($o === null || !is_object($o)) {}
        if ($me === null || !is_object($me)) {}
        if ($c === null || !is_object($c)) {}
        if ($c === null || !is_countable($c)) {}
        if ($u === null || !is_string($u)) {}
        if ($mixed === null || !is_string($mixed)) {}
        if ($plain === null || !is_string($plain)) {}
        if ($fs === null || !is_string($fs)) {}
        if ($this->prop === null || !is_string($this->prop)) {}
        if ($s === null || !is_string($i)) {}
        if ($s === null || !is_string(value: $s)) {}
        if ($s === null || !is_string($s, $s)) {}
        if (($s === null) || (!is_string($s))) {}
        if (($s === null || !is_string($s))) {}
        if ($s === null || !(is_string($s))) {}
        if ((($s)) === null || !is_string(($s))) {}
        if ($s === null || !is_string($s) || $i === 1) {}
        if ($i === 1 || $s === null || !is_string($s)) {}
        if ($i === 1 && ($s === null || !is_string($s))) {}
        $x = $s === null || !is_string($s) ? 1 : 2;
        return $s === null || !is_string($s);
    }

    public function ands(?string $s, array $a, ?array $na, int $i, bool $b, object $o, float $f, array|string $u, $mixed, ?int $ni)
    {
        if ($s && is_string($s)) {}
        if ($a && is_array($a)) {}
        if ($a && \Is_Array($a)) {}
        if ($na && is_array($na)) {}
        if ($i && is_int($i)) {}
        if ($ni && is_int($ni)) {}
        if ($b && is_bool($b)) {}
        if ($o && is_object($o)) {}
        if ($f && is_float($f)) {}
        if ($u && is_array($u)) {}
        if ($mixed && is_array($mixed)) {}
        if ($a && is_string($a)) {}
        if ($a && !is_array($a)) {}
        if ($a and is_array($a)) {}
        if (!$a && is_array($a)) {}
        if ($a && is_array($na)) {}
        if (($a) && (is_array($a))) {}
        if ($a && is_array($a) && $i) {}
        if ($i && $a && is_array($a)) {}
        if ($i && ($a && is_array($a))) {}
        if ($a && is_array(...)) {}
        if ($a && is_array($a, $a)) {}
        return $a && is_array($a);
    }

    public function locals()
    {
        $count = $this->resolve();
        if ($count === null || !is_int($count)) {
            return;
        }
        $list = $this->docArray();
        if ($list && is_array($list)) {}
        $text = $this->docNullable();
        if ($text === null || !is_string($text)) {}
    }

    /**
     * @param string|null $doc
     * @param array $items
     */
    public function docblocks($doc, $items)
    {
        if ($doc === null || !is_string($doc)) {}
        if ($items && is_array($items)) {}
    }

    private function resolve(): ?int { return 1; }
    /** @return array */
    private function docArray() { return []; }
    /** @return string|null */
    private function docNullable() { return null; }
}

function redundantOracleFn(?string $s, array $a) {
    return [$s === null || !is_string($s), $a && is_array($a)];
}
-----
<?php

namespace App;

class RedundantOracle
{
    private ?string $prop = null;

    public function ors(?string $s, ?int $i, ?float $f, ?bool $b, ?array $a, ?object $o, ?self $me, ?\Countable $c, string|int|null $u, $mixed, string $plain, null|false|string $fs)
    {
        if ($s === null) {}
        if (null === $s) {}
        if ($s === null) {}
        if ($s === null || !IS_STRING($s)) {}
        if ($s === NULL) {}
        if ($s == null || !is_string($s)) {}
        if ($s !== null || !is_string($s)) {}
        if ($s === null || is_string($s)) {}
        if ($s === null || !is_int($s)) {}
        if ($s === null or !is_string($s)) {}
        if ($i === null) {}
        if ($i === null) {}
        if ($i === null) {}
        if ($i === null || !is_numeric($i)) {}
        if ($f === null) {}
        if ($f === null) {}
        if ($b === null) {}
        if ($a === null) {}
        if ($a === null || !is_iterable($a)) {}
        if ($o === null) {}
        if ($me === null) {}
        if ($c === null) {}
        if ($c === null || !is_countable($c)) {}
        if ($u === null || !is_string($u)) {}
        if ($mixed === null || !is_string($mixed)) {}
        if ($plain === null || !is_string($plain)) {}
        if ($fs === null || !is_string($fs)) {}
        if ($this->prop === null || !is_string($this->prop)) {}
        if ($s === null || !is_string($i)) {}
        if ($s === null) {}
        if ($s === null || !is_string($s, $s)) {}
        if ($s === null) {}
        if (($s === null)) {}
        if ($s === null) {}
        if ((($s)) === null) {}
        if ($s === null || $i === 1) {}
        if ($i === 1 || $s === null || !is_string($s)) {}
        if ($i === 1 && ($s === null)) {}
        $x = $s === null ? 1 : 2;
        return $s === null;
    }

    public function ands(?string $s, array $a, ?array $na, int $i, bool $b, object $o, float $f, array|string $u, $mixed, ?int $ni)
    {
        if ($s) {}
        if ($a) {}
        if ($a && \Is_Array($a)) {}
        if ($na) {}
        if ($i) {}
        if ($ni) {}
        if ($b) {}
        if ($o) {}
        if ($f) {}
        if ($u && is_array($u)) {}
        if ($mixed && is_array($mixed)) {}
        if ($a && is_string($a)) {}
        if ($a && !is_array($a)) {}
        if ($a and is_array($a)) {}
        if (!$a && is_array($a)) {}
        if ($a && is_array($na)) {}
        if ($a) {}
        if ($a && $i) {}
        if ($i && $a && is_array($a)) {}
        if ($i && ($a)) {}
        if ($a && is_array(...)) {}
        if ($a && is_array($a, $a)) {}
        return $a;
    }

    public function locals()
    {
        $count = $this->resolve();
        if ($count === null) {
            return;
        }
        $list = $this->docArray();
        if ($list && is_array($list)) {}
        $text = $this->docNullable();
        if ($text === null || !is_string($text)) {}
    }

    /**
     * @param string|null $doc
     * @param array $items
     */
    public function docblocks($doc, $items)
    {
        if ($doc === null || !is_string($doc)) {}
        if ($items && is_array($items)) {}
    }

    private function resolve(): ?int { return 1; }
    /** @return array */
    private function docArray() { return []; }
    /** @return string|null */
    private function docNullable() { return null; }
}

function redundantOracleFn(?string $s, array $a) {
    return [$s === null, $a];
}
