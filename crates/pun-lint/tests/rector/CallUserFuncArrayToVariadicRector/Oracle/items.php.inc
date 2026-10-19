<?php

class Redirector { public function redirect($a) {} }

function spread(Redirector $r, $items, $list)
{
    call_user_func_array([...$r, 'redirect'], $items);
    call_user_func_array([$list[0]->assigned, 'redirect'], $items);
}
-----
<?php

class Redirector { public function redirect($a) {} }

function spread(Redirector $r, $items, $list)
{
    $r->redirect(...$items);
    call_user_func_array([$list[0]->assigned, 'redirect'], $items);
}
