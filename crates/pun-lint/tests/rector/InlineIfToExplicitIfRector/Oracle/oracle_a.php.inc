<?php

class InlineIfOracle
{
    private ?int $data = null;

    public function run($mixed, bool $flag, ?bool $maybe, int $n, array $list, string $s)
    {
        $flag && $n = 1;
        $flag || $n = 1;
        !$flag && $n = 1;
        !$flag || $n = 1;
        $maybe && $n = 1;
        $mixed && $n = 1;
        $n && $n = 1;
        $n > 1 && $n = 1;
        $n > 1 || $n = 1;
        $n >= 1 || $n = 1;
        $n == 1 || $n = 1;
        $n <=> 1 && $n = 1;
        isset($mixed) && $n = 1;
        isset($mixed) || $n = 1;
        empty($mixed) || $n = 1;
        $mixed instanceof \Countable && $n = count($mixed);
        $mixed instanceof \Countable || $n = 2;
        in_array($n, $list, true) || $list[] = $n;
        $flag && $maybe && $n = 1;
        $flag || $maybe || $n = 1;
        $flag && $maybe || $n = 1;
        ($flag && $maybe) || $n = 1;
        ($flag || $maybe) && $n = 1;
        $flag && ($n = 1);
        ($flag) && $this->go();
        ($flag && $this->go());
        $flag and $n = 1;
        $flag or $n = 1;
        $flag xor $n = 1;
        $flag && $n = 1 and $s = 'x';
        is_string($s) && $s .= 'a'; // trailing
        /* lead */ is_string($s) && $s .= 'b';
        is_string($s)
            && $s .= 'c';
        $flag && print 'x';
        $flag && $this->go() && $this->go();
        $flag && yield 1;
        $flag && throw new \Exception();
        $flag || throw new \Exception();
        (bool) $mixed && $n = 2;
        is_null($mixed) === false || $n = 3;
        $this->go() && $this->stop();
        $this->go() || $this->go();
        self::flag() || $n = 5;
        \defined('X') || define('X', 1);
        defined(...) && $n = 6;
        foreach ($list as $item) is_int($item) && $n += $item;
        $r = $flag && $n = 9;
        $f = fn () => $flag && $n = 10;
    }

    private function go(): bool { return true; }
    private function stop(): bool { return true; }
    /** @return bool */
    private function check() { return true; }
    private static function flag(): bool { return true; }
}

is_null($a) && $a = 5;
true || $a = 5;
-----
<?php

class InlineIfOracle
{
    private ?int $data = null;

    public function run($mixed, bool $flag, ?bool $maybe, int $n, array $list, string $s)
    {
        if ($flag) {
            $n = 1;
        }
        if (!$flag) {
            $n = 1;
        }
        if (!$flag) {
            $n = 1;
        }
        if ($flag) {
            $n = 1;
        }
        $maybe && $n = 1;
        $mixed && $n = 1;
        $n && $n = 1;
        if ($n > 1) {
            $n = 1;
        }
        if ($n <= 1) {
            $n = 1;
        }
        if ($n < 1) {
            $n = 1;
        }
        if ($n != 1) {
            $n = 1;
        }
        $n <=> 1 && $n = 1;
        if (isset($mixed)) {
            $n = 1;
        }
        if (!isset($mixed)) {
            $n = 1;
        }
        if (!empty($mixed)) {
            $n = 1;
        }
        if ($mixed instanceof \Countable) {
            $n = count($mixed);
        }
        if (!$mixed instanceof \Countable) {
            $n = 2;
        }
        if (!in_array($n, $list, true)) {
            $list[] = $n;
        }
        if ($flag && $maybe) {
            $n = 1;
        }
        if (!($flag || $maybe)) {
            $n = 1;
        }
        if (!($flag && $maybe)) {
            $n = 1;
        }
        if (!($flag && $maybe)) {
            $n = 1;
        }
        if ($flag || $maybe) {
            $n = 1;
        }
        if ($flag) {
            $n = 1;
        }
        if ($flag) {
            $this->go();
        }
        if ($flag) {
            $this->go();
        }
        $flag and $n = 1;
        $flag or $n = 1;
        $flag xor $n = 1;
        $flag && $n = 1 and $s = 'x';
        if (is_string($s)) {
            $s .= 'a';
        } // trailing
        /* lead */ if (is_string($s)) {
            $s .= 'b';
        }
        if (is_string($s)) {
            $s .= 'c';
        }
        if ($flag) {
            print 'x';
        }
        if ($flag && $this->go()) {
            $this->go();
        }
        if ($flag) {
            yield 1;
        }
        if ($flag) {
            throw new \Exception();
        }
        if (!$flag) {
            throw new \Exception();
        }
        if ((bool) $mixed) {
            $n = 2;
        }
        if (is_null($mixed) !== false) {
            $n = 3;
        }
        if ($this->go()) {
            $this->stop();
        }
        $this->go() || $this->go();
        if (!self::flag()) {
            $n = 5;
        }
        \defined('X') || define('X', 1);
        defined(...) && $n = 6;
        foreach ($list as $item) if (is_int($item)) {
            $n += $item;
        }
        $r = $flag && $n = 9;
        $f = fn () => $flag && $n = 10;
    }

    private function go(): bool { return true; }
    private function stop(): bool { return true; }
    /** @return bool */
    private function check() { return true; }
    private static function flag(): bool { return true; }
}

if (is_null($a)) {
    $a = 5;
}
if (!true) {
    $a = 5;
}
