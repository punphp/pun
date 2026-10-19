<?php
$a = $_SESSION[$key];
$_SESSION[$key] = $value;
$_SESSION[] = 1;
$a = $_SESSION['x']['y'];
isset($_SESSION['x'], $other);
unset($_SESSION['x'], $other);
session_start(['options']);
session_regenerate_id(true);
isset($other);
unset($other);
$a = $_session['key'];
isset($_session['key']);
unset($_session['key']);
$_session['key'] = $value;
$a = $_session;
SESSION_START();
Session_Regenerate_Id();
\session_start();
-----
<?php
$a = \Illuminate\Support\Facades\Session::get($key);
\Illuminate\Support\Facades\Session::put($key, $value);
$_SESSION[] = 1;
$a = \Illuminate\Support\Facades\Session::get('x')['y'];
\Illuminate\Support\Facades\Session::has('x');
\Illuminate\Support\Facades\Session::forget('x');
\Illuminate\Support\Facades\Session::start();
\Illuminate\Support\Facades\Session::regenerate();
isset($other);
unset($other);
$a = \Illuminate\Support\Facades\Session::get('key');
\Illuminate\Support\Facades\Session::has('key');
\Illuminate\Support\Facades\Session::forget('key');
\Illuminate\Support\Facades\Session::put('key', $value);
$a = \Illuminate\Support\Facades\Session::all();
SESSION_START();
Session_Regenerate_Id();
\Illuminate\Support\Facades\Session::start();
