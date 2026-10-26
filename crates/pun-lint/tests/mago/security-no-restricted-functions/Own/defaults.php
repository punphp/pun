<?php
namespace App;
use function Foo\bar as exec;
exec('ls');
\exec('ls');
shell_exec('ls');
curl_init();
\Curl_Exec($c);
strlen('a');
var_dump(1);
App\eval_me();
