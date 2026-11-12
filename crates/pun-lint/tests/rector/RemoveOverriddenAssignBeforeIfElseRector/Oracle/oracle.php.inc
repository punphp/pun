<?php

namespace App;

class OverriddenOracle
{
    public function a(bool $v, array $list)
    {
        // lead comment
        $a = 1; // trailing
        if ($v) {
            $a = 2;
        } else {
            $a = 3;
        }

        $b = 'x';
        if ($v) {
            echo 1;
            $b = 2;
        } else {
            $b = 3;
        }

        $c = 1;
        if ($v) {
            $c = 2;
        } elseif ($list) {
            $c = 4;
        } else {
            $c = 3;
        }

        $d = 1;
        if ($v) {
            $d = 2;
        } else if ($list) {
            $d = 4;
        } else {
            $d = 3;
        }

        $e = 1;
        if ($v) {
            $e += 2;
        } else {
            $e = 3;
        }

        $f = 1;
        if ($v) {
            [$f] = $list;
        } else {
            $f = 3;
        }

        $g = 1;
        if ($v) {
            ($g = 2);
        } else {
            $g = 3;
        }

        $h = 1;
        if ($v) {
            $h = fn () => $h;
        } else {
            $h = 3;
        }

        $i = 1;
        if ($v) {
            $i = 2;
        } else {
            $name = 'i';
            $i = $$name;
        }

        $j = 1;
        if ($v) {
            $j = 2;
        } else {
            $j = \extract($list);
        }

        $k = 1;
        if ($v) {
            $k = 2;
        } else {
            $k = Compact('list');
        }

        $l = 1;
        if ($v) {
            $l = 2;
        } else {
            $l = get_defined_vars();
        }

        $m = strlen('abc');
        if ($v) {
            $m = 2;
        } else {
            $m = 3;
        }

        $n = $o = 1;
        if ($v) {
            $n = 2;
        } else {
            $n = 3;
        }

        $_GET = [];
        if ($v) {
            $_GET = [1];
        } else {
            $_GET = [2];
        }

        $p = 1;
        if ($v):
            $p = 2;
        else:
            $p = 3;
        endif;

        $q = 1;
        if ($v) $q = 2; else $q = 3;

        $r = 1;
        if ($v) {
            if ($list) {
                $r = 2;
            }
        } else {
            $r = 3;
        }

        $s = 1;
        if ($s = $v) {
            $s = 2;
        } else {
            $s = 3;
        }

        $t = 1;
        $u = 1;
        if ($v) {
            $t = 2;
            $u = 2;
        } else {
            $u = 3;
            $t = 3;
        }

        $this->w = 1;
        if ($v) {
            $this->w = 2;
        } else {
            $this->w = 3;
        }

        $x = 1;
        if ($v) {
            $x = 2;
        } else {
        }

        $y = 1;

        if ($v) {
            $y = 2;
            return $y;
        } else {
            $y = 3;
        }

        $z =& $list;
        if ($v) {
            $z = 2;
        } else {
            $z = 3;
        }
        return [$a, $b, $c];
    }

    public function b(bool $v, &$ref)
    {
        $ref = 1;
        if ($v) {
            $ref = 2;
        } else {
            $ref = 3;
        }
        global $g;
        static $s;
        $g = 1;
        if ($v) {
            $g = 2;
        } else {
            $g = 3;
        }
        $s = 1;
        if ($v) {
            $s = 2;
        } else {
            $s = 3;
        }
        $only = 1;
        if ($v) {
            $only = 2;
        } else {
            $only = 3;
        }
    }

    public function c(bool $v)
    {
        foreach ([1] as $item) {
            $in = 1;
            if ($v) {
                $in = 2;
            } else {
                $in = 3;
            }
        }
        switch ($v) {
            case true:
                $sw = 1;
                if ($v) {
                    $sw = 2;
                } else {
                    $sw = 3;
                }
        }
        $fn = function () use ($v) {
            $cl = 1;
            if ($v) {
                $cl = 2;
            } else {
                $cl = "$cl";
            }
            $st = 1;
            if ($v) {
                $st = 2;
            } else {
                $st = "${v}";
            }
            $cs = 1;
            if ($v) {
                $cs = 2;
            } else {
                $cs = "${cs}";
            }
            $dy = 1;
            if ($v) {
                $dy = 2;
            } else {
                $dy = ${v};
            }
            $sp = 1;
            if (self::$sp) {
                $sp = 2;
            } else {
                $sp = 3;
            }
            $sq = 1;
            if (self::${'sq'} || $v::$$sq) {
                $sq = 2;
            } else {
                $sq = 3;
            }
            $pr = 1;
            if ($v) {
                $pr = new class { public $pr = 1; public $other; };
            } else {
                $pr = 3;
            }
        };
    }
}

$top = 1;
if (rand()) {
    $top = 2;
} else {
    $top = 3;
}
-----
<?php

namespace App;

class OverriddenOracle
{
    public function a(bool $v, array $list)
    {
        // trailing
        if ($v) {
            $a = 2;
        } else {
            $a = 3;
        }
        if ($v) {
            echo 1;
            $b = 2;
        } else {
            $b = 3;
        }

        $c = 1;
        if ($v) {
            $c = 2;
        } elseif ($list) {
            $c = 4;
        } else {
            $c = 3;
        }

        $d = 1;
        if ($v) {
            $d = 2;
        } else if ($list) {
            $d = 4;
        } else {
            $d = 3;
        }

        $e = 1;
        if ($v) {
            $e += 2;
        } else {
            $e = 3;
        }

        $f = 1;
        if ($v) {
            [$f] = $list;
        } else {
            $f = 3;
        }
        if ($v) {
            ($g = 2);
        } else {
            $g = 3;
        }

        $h = 1;
        if ($v) {
            $h = fn () => $h;
        } else {
            $h = 3;
        }

        $i = 1;
        if ($v) {
            $i = 2;
        } else {
            $name = 'i';
            $i = $$name;
        }

        $j = 1;
        if ($v) {
            $j = 2;
        } else {
            $j = \extract($list);
        }

        $k = 1;
        if ($v) {
            $k = 2;
        } else {
            $k = Compact('list');
        }

        $l = 1;
        if ($v) {
            $l = 2;
        } else {
            $l = get_defined_vars();
        }
        if ($v) {
            $m = 2;
        } else {
            $m = 3;
        }

        $n = $o = 1;
        if ($v) {
            $n = 2;
        } else {
            $n = 3;
        }

        $_GET = [];
        if ($v) {
            $_GET = [1];
        } else {
            $_GET = [2];
        }
        if ($v):
            $p = 2;
        else:
            $p = 3;
        endif;
        if ($v) $q = 2; else $q = 3;

        $r = 1;
        if ($v) {
            if ($list) {
                $r = 2;
            }
        } else {
            $r = 3;
        }

        $s = 1;
        if ($s = $v) {
            $s = 2;
        } else {
            $s = 3;
        }
        if ($v) {
            $t = 2;
            $u = 2;
        } else {
            $u = 3;
            $t = 3;
        }

        $this->w = 1;
        if ($v) {
            $this->w = 2;
        } else {
            $this->w = 3;
        }

        $x = 1;
        if ($v) {
            $x = 2;
        } else {
        }

        if ($v) {
            $y = 2;
            return $y;
        } else {
            $y = 3;
        }

        $z =& $list;
        if ($v) {
            $z = 2;
        } else {
            $z = 3;
        }
        return [$a, $b, $c];
    }

    public function b(bool $v, &$ref)
    {
        $ref = 1;
        if ($v) {
            $ref = 2;
        } else {
            $ref = 3;
        }
        global $g;
        static $s;
        $g = 1;
        if ($v) {
            $g = 2;
        } else {
            $g = 3;
        }
        $s = 1;
        if ($v) {
            $s = 2;
        } else {
            $s = 3;
        }
        if ($v) {
            $only = 2;
        } else {
            $only = 3;
        }
    }

    public function c(bool $v)
    {
        foreach ([1] as $item) {
            if ($v) {
                $in = 2;
            } else {
                $in = 3;
            }
        }
        switch ($v) {
            case true:
                if ($v) {
                    $sw = 2;
                } else {
                    $sw = 3;
                }
        }
        $fn = function () use ($v) {
            $cl = 1;
            if ($v) {
                $cl = 2;
            } else {
                $cl = "$cl";
            }
            if ($v) {
                $st = 2;
            } else {
                $st = "${v}";
            }
            $cs = 1;
            if ($v) {
                $cs = 2;
            } else {
                $cs = "${cs}";
            }
            $dy = 1;
            if ($v) {
                $dy = 2;
            } else {
                $dy = ${v};
            }
            if (self::$sp) {
                $sp = 2;
            } else {
                $sp = 3;
            }
            $sq = 1;
            if (self::${'sq'} || $v::$$sq) {
                $sq = 2;
            } else {
                $sq = 3;
            }
            if ($v) {
                $pr = new class { public $pr = 1; public $other; };
            } else {
                $pr = 3;
            }
        };
    }
}
if (rand()) {
    $top = 2;
} else {
    $top = 3;
}
