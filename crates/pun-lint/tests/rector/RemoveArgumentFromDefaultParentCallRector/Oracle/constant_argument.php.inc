<?php

const PUN_FOO = 'PUN_FOO';

class PunConstantBase
{
    public function run($a, $b = 'PUN_FOO')
    {
    }
}

class PunConstantChild extends PunConstantBase
{
    public function go()
    {
        parent::run(1, PUN_FOO);
    }
}
-----
<?php

const PUN_FOO = 'PUN_FOO';

class PunConstantBase
{
    public function run($a, $b = 'PUN_FOO')
    {
    }
}

class PunConstantChild extends PunConstantBase
{
    public function go()
    {
        parent::run(1);
    }
}
