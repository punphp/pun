<?php
if ($a) return;
if ($a) unrelated();
if ($a) report($exception);
if (!$a) report($exception);
if (($a = get()) && ($b = get())) { report($b); }
if ($a = get()) { report($other); }
-----
<?php
if ($a) return;
if ($a) unrelated();
report_if($a, $exception);
report_unless($a, $exception);
if (($a = get()) && ($b = get())) { report($b); }
report_if($a = get(), $other);
