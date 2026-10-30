<?php

class B
{
  public $a;
}

class C extends B
{
  public $a;
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
}
