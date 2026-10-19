<?php

class C
{
    public function m(int $p)
    {
        $ref = 1;
        for ($p = 0, $ref = &$p; rand();) {}
        if (rand()) {
        } elseif ($e1 = rand()) {
            for ($e1 = 0; rand();) {}
        }
        if (rand()):
        elseif ($e2 = rand()):
            $before = 1;
            for ($e2 = 0, $before = 0; rand();) {}
        endif;
        switch ($s = rand()) {
            case 1:
                for ($s = 0; rand();) {}
        }
        for ($i = 0; $j = rand(); $i++) {
            for ($i = 0, $j = 0; rand();) {}
        }
        foreach ([1] as $fv) {
            for ($fv = 0; rand();) {}
        }
        if (rand()) { $bx = 1; } else { { return; } }
        for ($bx = 0; rand();) {}
    }

    public function n()
    {
        if ($c = rand()) {}
        for ($c = 0; rand();) {}
        switch ($sw = rand()) { default: }
        for ($sw = 0; rand();) {}
        switch (rand()) { case 1: if (rand()) { break; } $b1 = 1; break; default: $b1 = 2; }
        for ($b1 = 0; rand();) {}
        switch (rand()) { case 1: $u1 = 1; unset($u1); break; default: $u1 = 2; }
        for ($u1 = 0; rand();) {}
        switch (rand()) { case 1: return; default: $x1 = 2; }
        for ($x1 = 0; rand();) {}
        $app[] = 1;
        [, $miss] = [1, 2];
        for ($app = 0, [, $miss] = [1, 2]; rand();) {}
        $bin = (rand() ? 1 : 2) + ($bb = 1);
        for ($bb = 0; rand();) {}
    }

    public int $hooked {
        set {
            for ($value = 0; rand();) {}
        }
    }

    public int $hooked2 {
        set(int $v) {
            for ($v = 0, $value = 0; rand();) {}
        }
    }
}

for ($undefined = 0; rand();) {}
-----
8: For loop initial assignment overwrites variable $p.
11: For loop initial assignment overwrites variable $e1.
16: For loop initial assignment overwrites variable $before.
16: For loop initial assignment overwrites variable $e2.
20: For loop initial assignment overwrites variable $s.
23: For loop initial assignment overwrites variable $i.
23: For loop initial assignment overwrites variable $j.
26: For loop initial assignment overwrites variable $fv.
29: For loop initial assignment overwrites variable $bx.
35: For loop initial assignment overwrites variable $c.
37: For loop initial assignment overwrites variable $sw.
43: For loop initial assignment overwrites variable $x1.
46: For loop initial assignment overwrites variable $app.
46: For loop initial assignment overwrites variable $miss.
48: For loop initial assignment overwrites variable $bb.
53: For loop initial assignment overwrites variable $value.
59: For loop initial assignment overwrites variable $v.
