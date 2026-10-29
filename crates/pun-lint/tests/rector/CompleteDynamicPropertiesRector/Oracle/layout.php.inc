<?php
class Inline { public function set() { $this->a = 1; } }
class Tabbed
{
	public function set()
	{
		$this->a = 1;
	}
}
class Commented
{
    // leading comment
    public function set() { $this->a = 1; }
}
-----
<?php
class Inline { /**
 * @var int
 */
public $a;
public function set() { $this->a = 1; } }
class Tabbed
{
	/**
     * @var int
     */
    public $a;
    public function set()
	{
		$this->a = 1;
	}
}
class Commented
{
    /**
     * @var int
     */
    public $a;
    // leading comment
    public function set() { $this->a = 1; }
}
