<?php

namespace App;

class InlineIfOracle2
{
    public function a(bool $flag, int $n)
    {
        $flag || exit(1);
    }

    public function b(bool $flag, int $n)
    {
        if ($n) $flag && $n = 7;
        if ($n):
            $flag || $n = 8;
        endif;
        if ($n) {
        } else $flag && $n = 7;
        while ($n) $flag && $n = 7;
        switch ($n) {
            case 1:
                $flag && $n = 7;
        }
    }

    public function d(bool $flag, int $n, $o)
    {
        $this->go() && $this->GO();
        $this->go() && $o->go();
        $o->go() && $o->go();
        $this->go() && ($this->go() || $this->go());
        $this->go() && $n = 1;
        !$this->go() && !$this->go();
        defined('A') && $n = 1;
        !defined('A') || $n = 1;
        DEFINED('A') && $n = 1;
        !(defined('A')) && $n = 1;
        (!defined('A')) && $n = 1;
        defined('A') === true && $n = 1;
        true && $n = 1;
        false || $n = 1;
        @is_dir('a') || mkdir('a');
        is_dir('a') || @mkdir('a');
        is_dir('a') or mkdir('a');
        !is_dir('a') && !mkdir('a') && throw new \RuntimeException();
        $flag && match (true) { default => $n = 4 };
        $flag && function () {
            return 1;
        };
        $flag && $n = [
            1,
            2,
        ];
        $n === 1 && $flag ? $n = 1 : $n = 2;
        $flag xor $n = 1;
        $flag & $n = 1;
        ! ! $flag || $n = 5;
        !($n > 1) || $n = 6;
        ($n > 1 xor $flag) || $n = 6;
    }

    private function go(): bool { return true; }
}

function inlineIfFn(bool $flag, int $n) {
  $flag && $n = 1;
}
-----
<?php

namespace App;

class InlineIfOracle2
{
    public function a(bool $flag, int $n)
    {
        if (!$flag) {
            exit(1);
        }
    }

    public function b(bool $flag, int $n)
    {
        if ($n) if ($flag) {
            $n = 7;
        }
        if ($n):
            if (!$flag) {
                $n = 8;
            }
        endif;
        if ($n) {
        } else if ($flag) {
            $n = 7;
        }
        while ($n) if ($flag) {
            $n = 7;
        }
        switch ($n) {
            case 1:
                if ($flag) {
                    $n = 7;
                }
        }
    }

    public function d(bool $flag, int $n, $o)
    {
        $this->go() && $this->GO();
        if ($this->go()) {
            $o->go();
        }
        $o->go() && $o->go();
        $this->go() && ($this->go() || $this->go());
        if ($this->go()) {
            $n = 1;
        }
        if (!$this->go()) {
            !$this->go();
        }
        defined('A') && $n = 1;
        !defined('A') || $n = 1;
        DEFINED('A') && $n = 1;
        !(defined('A')) && $n = 1;
        (!defined('A')) && $n = 1;
        if (defined('A') === true) {
            $n = 1;
        }
        if (true) {
            $n = 1;
        }
        if (!false) {
            $n = 1;
        }
        if (!@is_dir('a')) {
            mkdir('a');
        }
        if (!is_dir('a')) {
            @mkdir('a');
        }
        is_dir('a') or mkdir('a');
        if (!is_dir('a') && !mkdir('a')) {
            throw new \RuntimeException();
        }
        if ($flag) {
            match (true) { default => $n = 4 };
        }
        if ($flag) {
            function () {
                return 1;
            };
        }
        if ($flag) {
            $n = [
                1,
                2,
            ];
        }
        $n === 1 && $flag ? $n = 1 : $n = 2;
        $flag xor $n = 1;
        $flag & $n = 1;
        if (! $flag) {
            $n = 5;
        }
        if ($n > 1) {
            $n = 6;
        }
        if (!($n > 1 xor $flag)) {
            $n = 6;
        }
    }

    private function go(): bool { return true; }
}

function inlineIfFn(bool $flag, int $n) {
  if ($flag) {
      $n = 1;
  }
}
