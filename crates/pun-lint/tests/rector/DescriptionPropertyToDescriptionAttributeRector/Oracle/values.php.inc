<?php

use Illuminate\Console\Command;

class A extends Command
{
    protected $description = "Send \"mail\"\n";
}

class B extends Command
{
    protected $description = <<<TXT
        Send
          mail
        TXT;
}

class C extends Command
{
    protected $description = 'a' . 'b';
}

class D extends Command
{
    protected $description = self::class;
}

class E extends Command
{
    protected $description;
}
-----
<?php

use Illuminate\Console\Command;

#[\Illuminate\Console\Attributes\Description("Send \"mail\"\n")]
class A extends Command
{
}

#[\Illuminate\Console\Attributes\Description(<<<TXT
        Send
          mail
        TXT)]
class B extends Command
{
}

class C extends Command
{
    protected $description = 'a' . 'b';
}

class D extends Command
{
    protected $description = self::class;
}

class E extends Command
{
    protected $description;
}
