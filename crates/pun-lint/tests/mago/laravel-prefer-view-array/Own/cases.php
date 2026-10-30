<?php

namespace App;

use function Other\view as render;

function run($user)
{
    view('a')->with('user', $user);
    VIEW('a')->WITH('user', $user);
    \view('a')->with('user', $user);
    (view('a'))->with('user', $user);
    view('a')?->with('user', $user);
    view('a')->withErrors([]);
    view('a')->with('a', 1)->with('b', 2);
    render('a')->with('user', $user);
    Other\view('a')->with('user', $user);
    $view->with('user', $user);
}
