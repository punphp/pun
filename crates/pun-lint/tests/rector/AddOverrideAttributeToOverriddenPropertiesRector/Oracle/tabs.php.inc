<?php

class B
{
	public $a;
}

class C extends B
{
	public $a;

	/** Doc. */
	public $b;
}
-----
<?php

class B
{
	public $a;
}

class C extends B
{
	#[\Override]
    public $a;

	/** Doc. */
	public $b;
}
