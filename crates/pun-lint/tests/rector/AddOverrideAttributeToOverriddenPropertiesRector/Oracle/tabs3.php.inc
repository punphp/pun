<?php

#[\Attribute]
class M
{
}

class B
{
	public $a;
}

class C extends B
{
	#[M]
	public $a;
}
-----
<?php

#[\Attribute]
class M
{
}

class B
{
	public $a;
}

class C extends B
{
	#[M]
    #[\Override]
	public $a;
}
