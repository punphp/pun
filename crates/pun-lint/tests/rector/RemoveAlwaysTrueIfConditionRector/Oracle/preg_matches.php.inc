<?php

namespace App;

// PHPStan types `$matches` from the flags: with PREG_UNMATCHED_AS_NULL a group may be null
function run(string $pattern, string $subject, int $flags)
{
    preg_match_all('/\{[^}]*\}|([A-Za-z][\w-]*)="([^"]*)"|[^ \t]+/', $subject, $tokens, PREG_SET_ORDER | PREG_UNMATCHED_AS_NULL);
    foreach ($tokens as [, $key, $value]) {
        if ($key !== null && $value !== '') {
            echo $key;
        }
    }
    preg_match($pattern, $subject, $one);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 1;
        }
    }
    preg_match($pattern, $subject, $one, PREG_OFFSET_CAPTURE);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 2;
        }
    }
    preg_match($pattern, $subject, $one, PREG_UNMATCHED_AS_NULL);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 3;
        }
    }
    preg_match($pattern, $subject, $one, PREG_OFFSET_CAPTURE | PREG_UNMATCHED_AS_NULL);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 4;
        }
    }
    preg_match($pattern, $subject, $one, $flags);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 5;
        }
    }
    preg_match_all($pattern, $subject, $all);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 6;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_SET_ORDER);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 7;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_PATTERN_ORDER | PREG_OFFSET_CAPTURE);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 8;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_SET_ORDER | PREG_OFFSET_CAPTURE);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 9;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_UNMATCHED_AS_NULL);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 10;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_SET_ORDER | PREG_UNMATCHED_AS_NULL);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 11;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_SET_ORDER | PREG_OFFSET_CAPTURE | PREG_UNMATCHED_AS_NULL);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 12;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, $flags);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 13;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, 4);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 14;
            }
        }
    }
}
-----
<?php

namespace App;

// PHPStan types `$matches` from the flags: with PREG_UNMATCHED_AS_NULL a group may be null
function run(string $pattern, string $subject, int $flags)
{
    preg_match_all('/\{[^}]*\}|([A-Za-z][\w-]*)="([^"]*)"|[^ \t]+/', $subject, $tokens, PREG_SET_ORDER | PREG_UNMATCHED_AS_NULL);
    foreach ($tokens as [, $key, $value]) {
        if ($key !== null && $value !== '') {
            echo $key;
        }
    }
    preg_match($pattern, $subject, $one);
    foreach ($one as $v) {
        if ($subject !== '') {
            echo 1;
        }
    }
    preg_match($pattern, $subject, $one, PREG_OFFSET_CAPTURE);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 2;
        }
    }
    preg_match($pattern, $subject, $one, PREG_UNMATCHED_AS_NULL);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 3;
        }
    }
    preg_match($pattern, $subject, $one, PREG_OFFSET_CAPTURE | PREG_UNMATCHED_AS_NULL);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 4;
        }
    }
    preg_match($pattern, $subject, $one, $flags);
    foreach ($one as $v) {
        if ($v !== null && $subject !== '') {
            echo 5;
        }
    }
    preg_match_all($pattern, $subject, $all);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 6;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_SET_ORDER);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 7;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_PATTERN_ORDER | PREG_OFFSET_CAPTURE);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 8;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_SET_ORDER | PREG_OFFSET_CAPTURE);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 9;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_UNMATCHED_AS_NULL);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 10;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_SET_ORDER | PREG_UNMATCHED_AS_NULL);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 11;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, PREG_SET_ORDER | PREG_OFFSET_CAPTURE | PREG_UNMATCHED_AS_NULL);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 12;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, $flags);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 13;
            }
        }
    }
    preg_match_all($pattern, $subject, $all, 4);
    foreach ($all as $group) {
        foreach ($group as $v) {
            if ($v !== null && $subject !== '') {
                echo 14;
            }
        }
    }
}
