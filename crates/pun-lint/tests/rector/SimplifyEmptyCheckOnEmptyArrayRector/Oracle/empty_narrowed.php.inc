<?php

function a($v)
{
    if (!$v) {
        return;
    }
    if (!is_array($v)) {
        echo 1;
    }
    return empty($v);
}

$c = function ($v) {
    if (!is_array($v)) {
        return;
    }
    return empty($v);
};
-----
<?php

function a($v)
{
    if (!$v) {
        return;
    }
    if (!is_array($v)) {
        echo 1;
    }
    return empty($v);
}

$c = function ($v) {
    if (!is_array($v)) {
        return;
    }
    return $v === [];
};
