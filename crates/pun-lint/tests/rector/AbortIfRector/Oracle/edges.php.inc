<?php
if ($a) return;
if ($a) unrelated();
if ($a) { abort(500); /* trailing */ }
if ($a) abort(404, 'message');
if ($a): abort(403); endif;
if (!$a): abort(405); endif;
if (($a and $b)) { abort(400); }
if (($a = get()) && ($b = get())) { abort($b); }
if ($a = get()) { abort($other); }
if ($obj->x = get()) { abort($$name); }
if (Example::$x = get()) { abort($x); }
if ($array['x'] = get()) { abort($$name); }
if ($x = &$reference) { abort($x); }
-----
<?php
if ($a) return;
if ($a) unrelated();
if ($a) { abort(500); /* trailing */ }
abort_if($a, 404, 'message');
abort_if($a, 403);
abort_unless($a, 405);
abort_if($a and $b, 400);
if (($a = get()) && ($b = get())) { abort($b); }
abort_if($a = get(), $other);
abort_if($obj->x = get(), $$name);
if (Example::$x = get()) { abort($x); }
if ($array['x'] = get()) { abort($$name); }
abort_if($x = &$reference, $x);
