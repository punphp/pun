<?php
foreach (\iterator_to_array($a) as $x) {}
foreach (ITERATOR_TO_ARRAY($a) as $k => $x) {}
foreach (iterator_to_array(...) as $x) {}
foreach (Foo\iterator_to_array($a) as $x) {}
foreach (array_values(iterator_to_array($a)) as $x) {}
foreach ($o->iterator_to_array($a) as $x) {}
