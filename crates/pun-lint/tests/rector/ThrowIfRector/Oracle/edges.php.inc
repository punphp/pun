<?php
if ($a) return;
if ($a) unrelated();
if ($a) throw new \Exception($obj->name);
if ($a) throw new \Exception(Foo::$name);
if ($a) throw $existing;
if (!$a) throw $existing;
if ($a && $b) throw new \Exception($existing);
if ($a && ($other = get())) throw new \Exception($existing);
if ($a && ($obj->p = get())) throw new \Exception($existing);
if ($a && ($$name = get())) throw new \Exception($$name);
if ($a) throw new \Exception($obj?->name);
if (($a and ($x = get()))) throw new \Exception($x);
-----
<?php
if ($a) return;
if ($a) unrelated();
if ($a) throw new \Exception($obj->name);
if ($a) throw new \Exception(Foo::$name);
throw_if($a, $existing);
throw_unless($a, $existing);
throw_if($a && $b, new \Exception($existing));
throw_if($a && ($other = get()), new \Exception($existing));
throw_if($a && ($obj->p = get()), new \Exception($existing));
if ($a && ($$name = get())) throw new \Exception($$name);
throw_if($a, new \Exception($obj?->name));
throw_if($a and ($x = get()), new \Exception($x));
