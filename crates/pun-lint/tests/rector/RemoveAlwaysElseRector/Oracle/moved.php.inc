<?php

function multiline($a)
{
    if ($a) {
        return 1;
    } else {
        $x = [
            'a' => 1,

            'b' => 2,
        ];
        $y = <<<EOT
                heredoc
                  text
            EOT;
        $z = 'multi
    line';
        return foo(
            $x,
            $y,
            $z,
        );
    }
}

function keptWithComment($a, $b, $c)
{
    if ($a) {
        return 1;
    } elseif ($b) {
        echo 2;
    }
    // about c
    elseif ($c) {
        echo 3;
    }
}
-----
<?php

function multiline($a)
{
    if ($a) {
        return 1;
    }
    $x = [
        'a' => 1,

        'b' => 2,
    ];
    $y = <<<EOT
                heredoc
                  text
            EOT;
    $z = 'multi
    line';
    return foo(
        $x,
        $y,
        $z,
    );
}

function keptWithComment($a, $b, $c)
{
    if ($a) {
        return 1;
    }
    if ($b) {
        echo 2;
    } // about c
    elseif ($c) {
        echo 3;
    }
}
