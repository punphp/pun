<?php

namespace App;

use Illuminate\Console\Command;

class A extends Command
{
    protected $hidden = \true;
}

class B extends Command
{
    protected $hidden = True;
}

class C extends Command
{
    protected $hidden = 1;
}

class D extends Command
{
    protected $hidden = !false;
}

class E extends Command
{
    protected $hidden = \TRUE;
}
-----
<?php

namespace App;

use Illuminate\Console\Command;

#[\Illuminate\Console\Attributes\Hidden]
class A extends Command
{
}

#[\Illuminate\Console\Attributes\Hidden]
class B extends Command
{
}

class C extends Command
{
    protected $hidden = 1;
}

#[\Illuminate\Console\Attributes\Hidden]
class D extends Command
{
}

#[\Illuminate\Console\Attributes\Hidden]
class E extends Command
{
}
